pub mod delete;
pub mod empty_stream;
pub mod history_drawer;
pub mod index;
pub mod prompt_drawer;

use db::queries::{chats::Chat, chats_chunks::ChatChunks};

#[derive(PartialEq, Clone)]
pub struct ChatWithChunks {
    pub chat: Chat,
    pub chunks: Vec<ChatChunks>,
}
