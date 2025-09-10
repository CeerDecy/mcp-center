use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use mc_common::app::event::Event;
use mc_common::app::{AppState, Response};
use mc_db::model::{CreateFrom, McpServers, SettingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ListAllRequest {
    use_raw_endpoint: Option<bool>,
    page_size: Option<i64>,
    page_num: Option<i64>,
    query: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListAllResponse {
    servers: Vec<McpServers>,
    count: i64,
}

pub async fn list_all(
    State(state): State<AppState>,
    Query(request): Query<ListAllRequest>,
) -> Result<Json<Response>, (StatusCode, String)> {
    if request.page_size.is_some() ^ request.page_num.is_some() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Both page_size and page_num must be provided together or omitted together".to_string(),
        ));
    }

    let page_size = request.page_size.unwrap_or(0);
    let page_num = request.page_num.unwrap_or(0);

    let mcp_handler = match &state.handlers().mcp_handler {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't get MCP handler not found".to_string(),
            ));
        }
        Some(handler) => handler,
    };

    let settings_handler = match &state.handlers().system_settings_handler {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't get MCP handler not found".to_string(),
            ));
        }
        Some(handler) => handler,
    };

    let self_address = settings_handler
        .get_system_settings(SettingKey::SelfAddress)
        .await;

    // select mcp servers
    let mut servers = if page_size > 0 && page_num > 0 {
        mcp_handler
            .list_with_limit(request.query, page_size, (page_num - 1) * page_size)
            .await
            .map_err(|e| {
                tracing::error!("Failed to list mcp servers {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to list mcp servers".to_string(),
                )
            })?
    } else {
        mcp_handler.list_all().await.map_err(|e| {
            tracing::error!("Failed to list mcp servers {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list mcp servers".to_string(),
            )
        })?
    };

    // replace endpoint host
    if request.use_raw_endpoint.is_none() || !request.use_raw_endpoint.unwrap() {
        servers.iter_mut().for_each(|server| {
            server.endpoint = format!(
                "{self_address}/proxy/connect/{}/{}",
                server.name, server.tag
            );
        })
    }

    let count = mcp_handler.count().await.map_err(|e| {
        tracing::error!("Failed to count mcp servers {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to count mcp servers".to_string(),
        )
    })?;

    let data = serde_json::to_value(ListAllResponse { servers, count }).map_err(|e| {
        tracing::error!("Failed to parse mcp servers {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    Ok(Json(Response::new(Some(data))))
}
