pub mod form;
pub mod index;
pub mod model_table;
pub mod model_type;
use db::ModelType;

fn model_type(model_type: ModelType) -> String {
    match model_type {
        ModelType::LLM => "LLM".to_string(),
        ModelType::Image => "Image".to_string(),
        ModelType::Embeddings => "Embeddings".to_string(),
        ModelType::TextToSpeech => "TextToSpeech".to_string(),
    }
}
