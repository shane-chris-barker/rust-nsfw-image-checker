use axum::{response::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct PingResult {
    status: i32,
    result: bool
}

pub async fn ping() -> Json<PingResult> {
    let result = PingResult {
        status: 200,
        result: true
    };

    Json(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum:: {body::to_bytes};
    use serde_json::json;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_ping() {
        let response = ping().await.into_response();
        let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let json_value: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(json_value, json!({
            "status": 200,
            "result": true
        }));
    }
}
