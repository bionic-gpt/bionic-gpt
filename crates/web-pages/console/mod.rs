pub mod console_stream;
pub mod conversation;
pub mod empty_stream;
pub mod layout;
pub mod model_popup;
pub mod page;
pub mod prompt_drawer;
pub mod prompt_form;
pub mod response_timeline;
pub mod tool_call_timeline;
pub mod tools_modal;

use db::queries::{chats::Chat, chats_chunks::ChatChunks};
use openai_api::ToolCall;

#[derive(PartialEq, Clone, Debug)]
pub struct ChatWithChunks {
    pub chat: Chat,
    pub chunks: Vec<ChatChunks>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PendingChat {
    pub chat: Chat,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PendingChatState {
    PendingToolChats(Vec<Chat>, i32),
    PendingUserChat(Box<PendingChat>),
    None,
}

impl PendingChatState {
    pub fn shall_we_call_the_model(&self) -> bool {
        match self {
            PendingChatState::PendingToolChats(_, _) => true,
            PendingChatState::PendingUserChat(_) => true,
            PendingChatState::None => false,
        }
    }
}
