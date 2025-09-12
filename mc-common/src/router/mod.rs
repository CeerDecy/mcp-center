use crate::app::config::AppConfig;
use axum::Router;
use tower_http::cors::CorsLayer;

pub mod auth;
pub mod response;

pub type RouterHandler<T> = Box<dyn Fn(Router<T>, T, AppConfig) -> Router<T> + Send + Sync>;

pub struct RouterBuilder<S: Clone + Send + Sync + 'static> {
    router: Router<S>,
    handlers: Vec<RouterHandler<S>>,
}

impl<S: Clone + Send + Sync + 'static> Default for RouterBuilder<S> {
    fn default() -> Self {
        let router = Router::<S>::new();
        Self::new(router)
    }
}

impl<S: Clone + Send + Sync + 'static> RouterBuilder<S> {
    pub fn new(router: Router<S>) -> RouterBuilder<S> {
        RouterBuilder {
            router,
            handlers: vec![],
        }
    }
    pub fn with_register(mut self, handler: RouterHandler<S>) -> RouterBuilder<S> {
        self.handlers.push(handler);
        self
    }

    pub fn with_layer(mut self, handler: RouterHandler<S>) -> RouterBuilder<S> {
        self.handlers.push(handler);
        self
    }

    pub fn build<S2>(self, state: S, config: AppConfig) -> Router<S2> {
        let mut router = self.router.clone();
        for handle in self.handlers {
            router = handle(router.clone(), state.clone(), config.clone());
        }
        router.layer(CorsLayer::permissive()).with_state(state)
    }
}
