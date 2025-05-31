use serde::{Deserialize, Serialize};

/// The classification result returned after analyzing an image.
/// 
/// Each score represents the model's confidence (from 0.0 to 1.0) that the image
/// belongs to a given category. These scores are mutually independent.
#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct ClassificationResult {
    pub porn_score: f32,
    pub sexy_score: f32,
    pub hentai_score: f32,
    pub neutral_score: f32,
    pub drawing_score: f32,
}

/// The available NSFW content categories used in classification.
#[derive(Debug)]
pub enum Metric {
    Porn,
    Sexy,
    Hentai,
    Neutral,
    Drawings
}


/// A single category prediction output from the model.
///
/// Contains a metric label and the associated confidence score.
#[derive(Debug)]
pub struct ClassPrediction {
    pub metric: Metric,
    pub score: f32
}