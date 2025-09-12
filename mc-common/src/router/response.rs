use axum::Json;
use http::StatusCode;
use serde::Serialize;
use serde_json::Value;

pub type JsonResponse = Json<Response>;
#[derive(Serialize)]
pub struct Response {
    code: u16,
    message: String,
    data: Option<Value>,
}

impl Response {
    pub fn new<T: Serialize>(code: StatusCode, message: &str, data: Option<T>) -> JsonResponse {
        let value = match serde_json::to_value(data) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!("Failed to serialize JSON: {}", err);
                return Response::internal_server_error();
            }
        };
        Json(Response {
            code: code.as_u16(),
            message: message.to_string(),
            data: Some(value),
        })
    }

    pub fn success<T: Serialize>(data: T) -> JsonResponse {
        Response::new(StatusCode::OK, "OK", Some(data))
    }

    pub fn common(code: StatusCode, message: &str) -> JsonResponse {
        Response::new::<&'static str>(code, message, None)
    }

    pub fn error(message: &str) -> JsonResponse {
        Response::new::<&'static str>(StatusCode::INTERNAL_SERVER_ERROR, message, None)
    }

    pub fn internal_server_error() -> JsonResponse {
        Response::new::<&'static str>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            None,
        )
    }

    pub fn not_found<T: Serialize>() -> JsonResponse {
        Response::new::<T>(StatusCode::NOT_FOUND, "Not Found", None)
    }

    pub fn unauthorized<T: Serialize>() -> JsonResponse {
        Response::new::<T>(StatusCode::UNAUTHORIZED, "Unauthorized", None)
    }
}
