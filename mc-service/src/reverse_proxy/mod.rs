use crate::reverse_proxy::connection::ConnectionService;
use crate::reverse_proxy::message::MessageService;
use axum::Router;
use axum::http::StatusCode;
use axum::response::Response;
use bytes::Bytes;
use http_body_util::StreamBody;
use hyper::body::Frame;
use mc_common::app::AppState;
use mc_common::router;
use mc_common::router::auth;
use mc_common::router::auth::Permission;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;

pub mod connection;
pub mod message;

type ProxyResponse = Response<StreamBody<ReceiverStream<Result<Frame<Bytes>, std::io::Error>>>>;

pub fn build_error_stream_response(
    tx: Sender<Result<Frame<Bytes>, std::io::Error>>,
    stream: ReceiverStream<Result<Frame<Bytes>, std::io::Error>>,
    msg: String,
    status: StatusCode,
) -> ProxyResponse {
    tokio::task::spawn(async move {
        tx.send(Ok(Frame::data(Bytes::from(msg)))).await.unwrap();
    });

    let mut response_builder = Response::builder();
    response_builder = response_builder.status(status);
    response_builder.body(StreamBody::new(stream)).unwrap()
}

pub fn register_router(permission: Permission) -> router::RouterHandler<AppState> {
    Box::new(move |root, state, config| {
        let router = Router::new()
            .route_service(
                "/connect/{name}/{tag}",
                ConnectionService::new(state.https_client.clone(), state.mcp_cache.clone()),
            )
            .route_service(
                "/message/{name}/{tag}/{*subPath}",
                MessageService::new(state.https_client.clone(), state.mcp_cache.clone()),
            );

        let router = router::RouterBuilder::new(router)
            .with_layer(auth::layer_authorization(permission))
            .build(state, config);

        root.nest("/proxy", router)
    })
}
