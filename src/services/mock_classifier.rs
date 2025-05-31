use std::result;

use crate::services::classifier_trait::Classifier;
use crate::models::classified::ClassificationResult;
use crate::errors::AppError;

pub struct MockClassifier {
    pub default_result: ClassificationResult,
}

impl MockClassifier {
    pub fn new() -> Self {
        Self {
            default_result: ClassificationResult {
                porn_score: 0.1,
                sexy_score: 0.002,
                hentai_score: 0.005,
                neutral_score: 0.001,
                drawing_score: 0.002
            }
        }
    }

    pub fn with_result(result: ClassificationResult) -> Self {
        Self {
            default_result: result
        }
    }
}

#[async_trait::async_trait]
impl Classifier for MockClassifier {
    async fn classify(&self, _image_bytes: &[u8]) -> Result<ClassificationResult, AppError> {
        Ok(self.default_result.clone())
    }
}


