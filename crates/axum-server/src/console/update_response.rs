use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::chats;
use db::ChatStatus;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Chat {
    pub response: String,
    pub chat_id: i32,
}

pub async fn update_response(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(message): Form<Chat>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let chat_status = match message.response.as_str() {
        "Request aborted." => ChatStatus::Cancelled,
        "Error occurred while generating." => ChatStatus::Error,
        _ => ChatStatus::Success,
    };

    if message.validate().is_ok() {
        chats::update_chat()
            .bind(
                &transaction,
                &message.response,
                &chat_status,
                &message.chat_id,
            )
            .await?;
    }

    let chat = chats::chat()
        .bind(&transaction, &message.chat_id)
        .one()
        .await?;

    transaction.commit().await?;

    crate::layout::redirect(&ui_pages::routes::console::conversation_route(
        team_id,
        chat.conversation_id,
    ))
}
