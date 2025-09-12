use crate::app::AppState;
use crate::app::config::AppConfig;
use crate::router::RouterHandler;
use crate::router::response::{JsonResponse, Response as RouterResponse};
use axum::extract::{Request, State};
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use bitflags::bitflags;
use http::StatusCode;
use std::cmp::PartialEq;

bitflags! {
    #[derive(Clone,Copy)]
    pub struct Permission: u8 {
        const ADMIN = 0b01;
        const TOKEN = 0b10;
    }
}

pub fn layer_authorization(permission: Permission) -> RouterHandler<AppState> {
    Box::new(move |router, state, config| {
        router.layer(middleware::from_fn_with_state(
            (config.clone(), state.clone(), permission),
            authorization,
        ))
    })
}

async fn authorization(
    State((config, state, permission)): State<(AppConfig, AppState, Permission)>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, JsonResponse)> {
    if permission.is_empty() {
        tracing::info!("no need check permission");

        return Ok(next.run(req).await);
    }

    if let Some(key) = req.headers().get(http::header::AUTHORIZATION) {
        let mut apikey = key.to_str().unwrap();
        apikey = apikey.strip_prefix("Bearer ").unwrap_or(apikey);

        tracing::debug!("Authorization header set to: {apikey}");

        if permission.contains(Permission::ADMIN)
            && apikey == config.mcp_center.admin_token.as_str()
        {
            return Ok(next.run(req).await);
        }

        if permission.contains(Permission::TOKEN) {
            let handler = match &state.handlers().api_keys_handler {
                None => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        RouterResponse::internal_server_error(),
                    ));
                }
                Some(handler) => handler,
            };

            return match handler.find(apikey).await {
                Ok(_) => Ok(next.run(req).await),
                Err(sqlx::Error::RowNotFound) => {
                    tracing::error!("The API key is not permitted.");
                    Err((
                        StatusCode::UNAUTHORIZED,
                        RouterResponse::common(
                            StatusCode::UNAUTHORIZED,
                            "The API key is not permitted.",
                        ),
                    ))
                }
                Err(err) => {
                    tracing::error!("failed to select api key: {}", err);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        RouterResponse::internal_server_error(),
                    ))
                }
            };
        }

        return Err((
            StatusCode::UNAUTHORIZED,
            RouterResponse::common(StatusCode::UNAUTHORIZED, "The API key is not permitted."),
        ));
    }

    tracing::error!("Authorization header not found");

    Err((
        StatusCode::UNAUTHORIZED,
        RouterResponse::common(StatusCode::UNAUTHORIZED, "Authorization header not found"),
    ))
}
