mod mcp_server;

use axum::routing::{get, post, put};
use mc_common::app::AppState;
use mc_common::router;
pub use mcp_server::*;

pub fn register_router() -> router::RouterHandler<AppState> {
    Box::new(|router| {
        tracing::info!("Register router");
        tracing::info!("GET\t/api/registry/mcp-server");
        tracing::info!("POST\t/api/registry/mcp-server");
        tracing::info!("PUT\t/api/registry/mcp-server/{{mcp_name}}");

        router
            .route("/api/registry/mcp-server", get(list_all))
            .route("/api/registry/mcp-server", post(register_mcp_server))
            .route(
                "/api/registry/mcp-server/{mcp_name}",
                put(update_mcp_server),
            )
    })
}
