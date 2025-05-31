use crate::services::classifier_trait::Classifier;
use crate::models::classified::ClassificationResult;
use crate::services::classification;
use crate::errors::AppError;

/// A concrete implementation of the `Classifier` trait using an NSFW detection model.
///
/// This classifier wraps an instance of a pre-trained NSFW ONNX model to evaluate
/// whether an image contains explicit or safe content.
pub struct NsfwClassifier {
    model: nsfw::Model,
}

impl NsfwClassifier {
    /// Creates a new `NsfwClassifier` with the given NSFW model.
    ///
    /// # Arguments
    ///
    /// * `model` - An instance of the loaded NSFW ONNX model.
    pub fn new(model: nsfw::Model) -> Self {
        Self { model }
    }
}

#[async_trait::async_trait]
impl Classifier for NsfwClassifier {
    /// Asynchronously classifies an image by running it through the NSFW model.
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - A byte slice representing the image to be analyzed.
    ///
    /// # Returns
    ///
    /// A `ClassificationResult` containing scores for each content category,
    /// or an `AppError` if the image couldn't be processed.
    async fn classify(&self, image_bytes: &[u8]) -> Result<ClassificationResult, AppError> {
        let img = classification::load_image_from_bytes(&bytes::Bytes::from(image_bytes.to_vec()))?;
        let rgba_img = img.to_rgba8();
        classification::classify(&self.model, &rgba_img)
    }
}