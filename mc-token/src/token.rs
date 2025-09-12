use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use mc_common::app::AppState;
use mc_common::router::response::{JsonResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AdminLoginRequest {
    pub username: String,
    pub token: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AdminLoginResponse {
    pub code: Option<u16>,
    pub message: Option<String>,
}

pub async fn admin_login(
    State(_state): State<AppState>,
    Json(request): Json<AdminLoginRequest>,
) -> Result<JsonResponse, (StatusCode, JsonResponse)> {
    if request.username != "admin" {
        return Err((
            StatusCode::UNAUTHORIZED,
            Response::common(StatusCode::UNAUTHORIZED, "only admin can login"),
        ));
    }
    let env_token = std::env::var("MCP_ADMIN_TOKEN").unwrap_or_else(|e| {
        tracing::warn!("MCP_ADMIN_TOKEN environment variable is not set: {}", e);
        String::from("")
    });

    if let Some(token) = request.token {
        if token != env_token {
            tracing::error!(
                "MCP_ADMIN_TOKEN environment variable is {}, request token is {}",
                env_token,
                token
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::error("Invalid token"),
            ));
        }
        return Ok(Response::success(""));
    }

    tracing::warn!("only support admin token in current version");
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Response::error("only support admin token in current version"),
    ))
}
