use axum::Extension;
use axum::{response::Json};
use axum_extra::extract::Multipart;
use bytes::Bytes;
use crate::errors::AppError;
use crate::models::classified;
use classified::ClassificationResult;
use std::sync::Arc;
use crate::services::classifier_trait::Classifier;

/// Handles an HTTP multipart upload, extracts the image, and returns classification scores.
///
/// This endpoint expects a multipart form-data request with an `image` field.
/// The image is passed to a classifier implementation (e.g., NSFW model),
/// and the resulting scores are returned in JSON format.
///
/// # Arguments
/// * `model` - An `Arc`-wrapped classifier implementation injected via Axum's `Extension`.
/// * `multipart` - A multipart form containing the uploaded image.
///
/// # Returns
/// * `200 OK` with `ClassificationResult` in JSON format on success.
/// * `400 Bad Request` if no image is provided or parsing fails.
/// * `500 Internal Server Error` if classification fails internally.
pub async fn classify_image(
    Extension(model): Extension<Arc<dyn Classifier>>,
    mut multipart: Multipart
) -> Result<Json<ClassificationResult>, AppError> {

    let mut img_data: Option<Bytes> = None;
    while let Some(field) = multipart.next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read field {e}")))? {
        
        let name = field.name().unwrap_or("").to_string();
        if name == "image" {
            img_data = Some(field.bytes().await.map_err(|_| AppError::BadRequest("Failed to read image bytes".into()))?);
            break;
        }
    }

    let img_data = img_data.ok_or_else(|| AppError::BadRequest("No image field found".into()))?;
    let result = model.classify(img_data.as_ref()).await?;

    Ok(Json(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::mock_classifier::MockClassifier;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_classify_image() {
        let mock_classifier: Arc<dyn Classifier> = Arc::new(MockClassifier::new());
        
        // Test your handler or service logic without any files!
        let result = mock_classifier.classify(b"fake image data").await.unwrap();
        assert_eq!(result.neutral_score, 0.001);
    }
}
