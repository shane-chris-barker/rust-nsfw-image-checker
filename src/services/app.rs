use axum::{Router, Extension};
use std::sync::Arc;
use crate::services::classification;
use crate::services::classifier_trait::Classifier;
use crate::errors::AppError;
use crate::services::nsfw_classifier::NsfwClassifier;

pub async fn build_app() -> Result<Router, AppError> {
    let base = env!("CARGO_MANIFEST_DIR");
    let model_path = format!("{}/models/model.onnx", base);
    let model_bytes = classification::read_model(model_path)?;
    let model = classification::load_model(model_bytes)?;
    let nsfw_classifier: Arc<dyn Classifier> = Arc::new(NsfwClassifier::new(model));

    Ok(Router::new()
        .route("/classify", axum::routing::post(crate::handlers::classify::classify_image))
        .route("/ping", axum::routing::get(crate::handlers::ping::ping))
        .layer(Extension(nsfw_classifier))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_app() {
        let app = build_app().await;
        assert!(app.is_ok());
    }
}
