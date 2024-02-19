pub mod delete;
pub mod history_drawer;
pub mod index;
pub mod prompt_drawer;

use db::queries::{chats::Chat, chats_chunks::ChatChunks};
pub use index::index;

#[derive(PartialEq)]
pub struct ChatWithChunks {
    pub chat: Chat,
    pub chunks: Vec<ChatChunks>,
}
