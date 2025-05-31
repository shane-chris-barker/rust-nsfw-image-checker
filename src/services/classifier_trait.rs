use crate::models::{classified};
use classified::ClassificationResult;
use crate::errors::AppError;


/// Trait for image classification models.
///
/// Any type implementing this trait must be able to asynchronously classify
/// an image from a byte slice and return a `ClassificationResult`
#[async_trait::async_trait]
pub trait Classifier: Send + Sync {
    async fn classify(&self, image_bytes: &[u8]) -> Result<ClassificationResult, AppError>;
}