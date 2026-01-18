pub mod page;
pub mod upsert;

use db::OpenapiSpecCategory;

pub fn category_label(category: OpenapiSpecCategory) -> &'static str {
    match category {
        OpenapiSpecCategory::WebSearch => "Web Search",
        OpenapiSpecCategory::CodeSandbox => "CodeSandbox",
        OpenapiSpecCategory::Application => "Application",
    }
}
