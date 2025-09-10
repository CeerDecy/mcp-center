mod find;
mod register;
mod update;

use axum::routing::{get, post};
use mc_common::app::AppState;
use mc_common::router;

pub fn register_router() -> router::RouterHandler<AppState> {
    Box::new(|router| {
        router
            .route("/api/registry/mcp-server", get(find::list_all))
            .route(
                "/api/registry/mcp-server",
                post(register::register_mcp_server),
            )
            .route(
                "/api/registry/mcp-server/update",
                post(update::update_mcp_server),
            )
    })
}
