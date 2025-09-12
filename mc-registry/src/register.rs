use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use mc_common::app::AppState;
use mc_common::app::event::Event;
use mc_common::router::response::{JsonResponse, Response};
use mc_db::model::{CreateFrom, McpServers};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct McpRegisterRequest {
    pub name: String,
    pub tag: String,
    pub endpoint: String,
    pub transport_type: String,
    pub description: String,
    pub create_from: Option<String>,
    pub extra: Option<serde_json::Value>,
}

pub async fn register_mcp_server(
    State(state): State<AppState>,
    Json(server): Json<McpRegisterRequest>,
) -> Result<JsonResponse, (StatusCode, JsonResponse)> {
    let mcp_handler = match &state.handlers().mcp_handler {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::error("Can't get MCP handler not found"),
            ));
        }
        Some(handler) => handler,
    };

    let res = mcp_handler
        .create(&McpServers {
            id: Uuid::new_v4(),
            name: server.name.clone(),
            tag: server.tag.clone(),
            endpoint: server.endpoint.clone(),
            transport_type: server.transport_type.clone(),
            description: server.description.clone(),
            create_from: if server.create_from.is_some() {
                server.create_from.unwrap().clone()
            } else {
                CreateFrom::Register.to_string()
            },
            extra: server.extra.clone(),
            disabled: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: None,
        })
        .await
        .map_err(|e| {
            tracing::error!("Failed to create mcp server {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::error("Failed to create mcp server"),
            )
        })?;

    tokio::task::spawn(async move {
        if let Err(err) = state.event_sender.send(Event::CreateOrUpdate {
            mcp_name: server.name.clone(),
            tag: server.tag.clone(),
            endpoint: server.endpoint.clone(),
        }) {
            tracing::error!("Failed to send event {}", err);
        }
        tracing::info!("MCP server {} registered", server.name);
    });

    Ok(Response::success(res))
}
