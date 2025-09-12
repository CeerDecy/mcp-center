use axum::Router;
use axum::routing::post;
use mc_common::app::AppState;
use mc_common::router;
use mc_common::router::RouterBuilder;
use mc_common::router::auth::{Permission, layer_authorization};

mod token;

pub fn register_router(permission: Permission) -> router::RouterHandler<AppState> {
    Box::new(move |root, state, config| {
        let mut router = Router::new();
        router = RouterBuilder::new(router)
            .with_layer(layer_authorization(permission))
            .build(state, config);

        router = router.route("/admin/login", post(token::admin_login));

        root.nest("/api/v1/user", router)
    })
}
