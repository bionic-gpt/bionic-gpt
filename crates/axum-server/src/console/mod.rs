mod index;
mod send_message;
mod update_response;
use axum::{
    routing::{get, post},
    Router,
};

use ui_components::routes::console::{INDEX, SEND_MESSAGE, UPDATE_RESPONSE};

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(SEND_MESSAGE, post(send_message::send_message))
        .route(UPDATE_RESPONSE, post(update_response::update_response))
}
