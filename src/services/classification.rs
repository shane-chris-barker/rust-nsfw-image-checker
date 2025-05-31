use std::fs;
use std::io::Cursor;
use image::{load_from_memory, RgbaImage, DynamicImage};
use crate::errors::AppError;
use nsfw::model::{Metric, Classification};
use nsfw::{create_model, examine};
use crate::models::classified::{ClassPrediction, Metric as ClassifiedMetric, ClassificationResult};
use bytes::Bytes;
use nsfw::Model;

/// Loads an image from raw byte data.
///
/// Returns a `DynamicImage` if the bytes represent a valid image.
/// Returns a `BadRequest` error if the data is invalid or unreadable.
pub fn load_image_from_bytes(img_data: &Bytes) -> Result<DynamicImage, AppError> {
    load_from_memory(&img_data).map_err(|e| AppError::BadRequest(format!("Invalid image data {e}")))
}

/// Reads the ONNX model file from the provided path.
///
/// Returns a byte vector if successful, or an `Internal` error if the file cannot be read.
pub fn read_model<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<u8>, AppError> {
    fs::read(path).map_err(|e| AppError::Internal(format!("Model file missing. Run ./download_model.sh to fetch the required model before starting the app. {e}")))
}

/// Loads the NSFW classification model from raw bytes.
///
/// Returns a `Model` instance on success or an `Internal` error on failure.
pub fn load_model(data:Vec<u8>) -> Result<Model, AppError> {
    let cursor = Cursor::new(data);
    create_model(cursor).map_err(|e| AppError::Internal(format!("Failed to load model {e}")))
}

/// Converts internal classification predictions into the API-facing result format.
///
/// Takes a slice of `ClassPrediction` and returns a `ClassificationResult` with scores populated accordingly.
pub fn map_classifications_to_result(classifications: &[ClassPrediction]) -> ClassificationResult {
    let mut result = ClassificationResult::default();
    for class in classifications {
        match class.metric {
            ClassifiedMetric::Porn => result.porn_score = class.score,
            ClassifiedMetric::Sexy => result.sexy_score = class.score,
            ClassifiedMetric::Hentai => result.hentai_score = class.score,
            ClassifiedMetric::Neutral => result.neutral_score = class.score,
            ClassifiedMetric::Drawings => result.drawing_score = class.score,
        }
    }
    result
}

/// Classifies an RGBA image using the loaded NSFW model.
///
/// Returns a `ClassificationResult` with the scores for each category.
/// Returns an `Internal` error if classification fails
pub fn classify(model: &Model, rgba_img: &RgbaImage) -> Result<ClassificationResult, AppError> {
    let classifications: Vec<Classification> = examine(&model, &rgba_img)
        .map_err(|_| AppError::Internal(format!("Failed to classify the image")))?;
    let predictions: Vec<ClassPrediction> = classifications.into_iter()
        .map(|c| ClassPrediction {
            metric: match c.metric {
                Metric::Porn => ClassifiedMetric::Porn,
                Metric::Sexy => ClassifiedMetric::Sexy,
                Metric::Hentai => ClassifiedMetric::Hentai,
                Metric::Neutral => ClassifiedMetric::Neutral,
                Metric::Drawings => ClassifiedMetric::Drawings,
            },
            score: c.score,
        })
        .collect();

   let result = map_classifications_to_result(&predictions);
   Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    #[test]
    fn test_load_image_from_bytes_handles_invalid_data() {
        let invalid_data = Bytes::from("This is not an image");
        let result = load_image_from_bytes(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_classifiy_handles_empty_classifications() {
        let dummy_model_data = vec![];
        let cursor = Cursor::new(dummy_model_data);
        let model = create_model(cursor);
        assert!(model.is_err());
    }

    #[test]
    fn test_it_maps_correctly() {
        let result = ClassificationResult::default();
        assert_eq!(result.porn_score, 0.0);
        assert_eq!(result.sexy_score, 0.0);
        assert_eq!(result.hentai_score, 0.0);
        assert_eq!(result.drawing_score, 0.0);
        assert_eq!(result.neutral_score, 0.0);
    }

    #[test]
    fn test_read_model_handles_missing_model() {
        let error_contains_string = "Model file missing. Run ./download_model.sh to fetch the required model";
        let result = read_model("fake/model/path");
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::Internal(msg) => assert!(msg.contains(error_contains_string)),
            _ => panic!("Expected internal error"),
        }
    }
}
