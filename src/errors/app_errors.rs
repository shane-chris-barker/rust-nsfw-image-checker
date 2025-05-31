use axum:: {
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Internal(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": msg}))
            ).into_response(),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": msg}))
            ).into_response()
        }
    }
}

#[cfg (test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use axum::http::StatusCode;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_it_returns_bad_response() {
        let err_string = "Invalid Input";
        let err = AppError::BadRequest(err_string.to_string());
        let response = err.into_response();
        let status = response.status();
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let json : Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json, json!({"error": err_string}));        
    }

    #[tokio::test]
    async fn test_it_returns_internal_error_response() {
        let err_string = "Something Went Wrong";
        let err = AppError::Internal(err_string.to_string());
        let response = err.into_response();
        let status = response.status();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json, json!({"error": err_string})); 
    }

    #[test]
    fn test_that_debug_is_covered() {
        let err = AppError::BadRequest("Debug".into());
        let _ = format!("{:?}", err);
    }
}
