use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries::chats;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::console::UpdateResponse;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Chat {
    pub response: String,
    pub chat_id: i32,
}

pub async fn update_response(
    UpdateResponse { team_id }: UpdateResponse,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(message): Form<Chat>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let chat = chats::chat()
        .bind(&transaction, &message.chat_id)
        .one()
        .await?;

    super::super::layout::redirect(
        &web_pages::routes::console::Conversation {
            team_id,
            conversation_id: chat.conversation_id,
        }
        .to_string(),
    )
}
