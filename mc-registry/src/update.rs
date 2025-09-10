use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use mc_common::app::{AppState, Response};
use mc_db::model::McpServers;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UpdateMcpRequest {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub endpoint: String,
    pub transport_type: String,
    pub description: String,
    pub create_from: String,
    pub extra: Option<serde_json::Value>,
    pub disabled: bool,
}

pub async fn update_mcp_server(
    State(state): State<AppState>,
    Json(request): Json<UpdateMcpRequest>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let mcp_handler = match &state.handlers().mcp_handler {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't get MCP handler not found".to_string(),
            ));
        }
        Some(handler) => handler,
    };

    let server = mcp_handler
        .update(McpServers {
            id: request.id.clone(),
            name: request.name.clone(),
            tag: request.tag.clone(),
            endpoint: request.endpoint.clone(),
            transport_type: request.transport_type.clone(),
            description: request.description.clone(),
            create_from: request.create_from.clone(),
            extra: request.extra.clone(),
            disabled: request.disabled.clone(),
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: None,
        })
        .await
        .map_err(|err| {
            tracing::error!(?err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't update MCP Server".to_string(),
            )
        })?;

    let value = serde_json::to_value(server).map_err(|err| {
        tracing::error!("Failed to parse mcp servers {}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    tracing::info!("mcp server successfully updated, {:?}", request);

    Ok(Json(Response::new(Some(value))))
}
