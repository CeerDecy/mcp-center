use axum::routing::{get, post};
use mc_common::app::AppState;
use mc_common::router;

mod setting;

pub fn register_router() -> router::RouterHandler<AppState> {
    Box::new(|router| {
        tracing::info!("Settings router");
        tracing::info!("GET\t/api/settings");
        tracing::info!("POST\t/api/settings");

        router
            .route("/api/settings", get(setting::list_settings))
            .route("/api/settings", post(setting::update_settings))
    })
}
