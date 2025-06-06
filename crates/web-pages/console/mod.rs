pub mod console_stream;
pub mod conversation;
pub mod empty_stream;
pub mod history_drawer;
pub mod index;
pub mod layout;
pub mod model_popup;
pub mod prompt_drawer;
pub mod prompt_form;
pub mod tools_modal;

use db::queries::{chats::Chat, chats_chunks::ChatChunks};
use openai_api::ToolCall;

#[derive(PartialEq, Clone)]
pub struct ChatWithChunks {
    pub chat: Chat,
    pub chunks: Vec<ChatChunks>,
}

#[derive(PartialEq, Clone)]
pub struct PendingChat {
    pub chat: Chat,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(PartialEq, Clone)]
pub enum PendingChatState {
    PendingToolChats(Vec<Chat>, i32),
    PendingUserChat(Box<PendingChat>),
    None,
}
