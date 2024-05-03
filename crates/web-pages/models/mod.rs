pub mod delete;
pub mod form;
pub mod index;
pub mod model_table;
pub mod model_type;
use db::ModelType;

fn model_type(model_type: ModelType) -> String {
    if model_type == ModelType::LLM {
        "LLM".to_string()
    } else {
        "Embeddings".to_string()
    }
}
