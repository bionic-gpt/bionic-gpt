mod conversation;
mod delete;
mod index;
mod new_chat;
mod send_message;
mod update_response;
use axum::{
    routing::{get, post},
    Router,
};

use ui_pages::routes::console::{
    CONVERSATION, DELETE, INDEX, NEW_CHAT, SEND_MESSAGE, UPDATE_RESPONSE,
};

pub fn routes() -> Router {
    Router::new()
        .route(CONVERSATION, get(conversation::conversation))
        .route(INDEX, get(index::index))
        .route(SEND_MESSAGE, post(send_message::send_message))
        .route(UPDATE_RESPONSE, post(update_response::update_response))
        .route(NEW_CHAT, post(new_chat::new_chat))
        .route(DELETE, post(delete::delete))
}
