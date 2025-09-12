use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use mc_common::app::AppState;
use mc_common::router::response::{JsonResponse, Response};
use mc_db::model::McpServers;
use serde::{Deserialize, Serialize};
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

    let server = mcp_handler
        .update(McpServers {
            id: request.id,
            name: request.name.clone(),
            tag: request.tag.clone(),
            endpoint: request.endpoint.clone(),
            transport_type: request.transport_type.clone(),
            description: request.description.clone(),
            create_from: request.create_from.clone(),
            extra: request.extra.clone(),
            disabled: request.disabled,
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: None,
        })
        .await
        .map_err(|err| {
            tracing::error!(?err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::error("Can't update MCP Server"),
            )
        })?;

    tracing::info!("mcp server successfully updated, {:?}", request);

    Ok(Response::success(server))
}
