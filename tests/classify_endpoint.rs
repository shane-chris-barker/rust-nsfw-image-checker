use axum:: {
    body::Body,
    body::to_bytes,
    http::{Request, StatusCode},
    Router
};
use tower::ServiceExt;
use nsfw_checker::services::mock_classifier::MockClassifier;
use nsfw_checker::services::classifier_trait::Classifier;
use nsfw_checker::models::classified::ClassificationResult;
use axum::Extension;
use nsfw_checker::handlers::classify::classify_image;
use std::{fs, sync::Arc};

#[tokio::test]async fn test_classify_endpoint_with_real_image() {
    let mock_result = ClassificationResult {
        porn_score: 0.9,
        sexy_score: 0.05,
        hentai_score: 0.01,
        neutral_score: 0.03,
        drawing_score: 0.01,
    };
    
    let mock_classifier: Arc<dyn Classifier> = Arc::new(MockClassifier::with_result(mock_result.clone()));

    let app = Router::new()
        .route("/classify", axum::routing::post(classify_image))
        .layer(Extension(mock_classifier));

    let image_bytes = fs::read("tests/assets/test.png").expect("Failed to read test image");
    let boundary = "BOUNDARY";
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n");
    body.extend_from_slice(b"Content-Type: image/jpeg\r\n\r\n");
    body.extend_from_slice(&image_bytes);
    body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .uri("/classify")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.expect("Request failed");
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let classification: ClassificationResult = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");

    // Assert the mock result is returned
    assert_eq!(classification.porn_score, 0.9);
    // add more assertions as needed...
}