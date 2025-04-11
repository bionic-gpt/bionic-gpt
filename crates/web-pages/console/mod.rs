pub mod console_stream;
pub mod conversation;
pub mod delete;
pub mod empty_stream;
pub mod history_drawer;
pub mod index;
pub mod layout;
pub mod model_popup;
pub mod prompt_drawer;
pub mod prompt_form;

use db::queries::{chats::Chat, chats_chunks::ChatChunks};
use openai_api::ToolResult;

#[derive(PartialEq, Clone)]
pub struct ChatWithChunks {
    pub chat: Chat,
    pub chunks: Vec<ChatChunks>,
}

impl ChatWithChunks {
    pub fn get_function_call_results(&self) -> Option<ToolResult> {
        // Parse the function call results

        if let Some(function_call_results) = &self.chat.function_call_results {
            serde_json::from_str(function_call_results).unwrap_or(None)
        } else {
            None
        }
    }
}
