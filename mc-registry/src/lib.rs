mod find;
mod register;
mod update;

use axum::Router;
use axum::routing::{get, post};
use mc_common::app::AppState;
use mc_common::router;
use mc_common::router::RouterHandler;
use mc_common::router::auth::{Permission, layer_authorization};

pub fn register_router(permission: Permission) -> RouterHandler<AppState> {
    Box::new(move |root, state, config| {
        let app = Router::new()
            .route("/mcp-server", get(find::list_all))
            .route("/mcp-server", post(register::register_mcp_server))
            .route("/mcp-server/update", post(update::update_mcp_server));

        let router = router::RouterBuilder::new(app)
            .with_layer(layer_authorization(permission))
            .build(state, config);

        root.nest("/api/v1/registry", router)
    })
}
