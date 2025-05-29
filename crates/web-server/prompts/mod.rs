mod conversation;
mod delete;
mod delete_conv;
mod form;
mod image;
mod loaders;
mod my_prompts;
mod new_chat;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(loaders::index_loader)
        .typed_get(loaders::new_assistant_loader)
        .typed_get(loaders::edit_assistant_loader)
        .typed_get(my_prompts::my_prompts)
        .typed_get(conversation::conversation)
        .typed_get(image::image)
        .typed_get(new_chat::new_chat)
        // Actions
        .typed_post(form::upsert)
        .typed_post(delete_conv::delete)
        .typed_post(delete::delete)
}
