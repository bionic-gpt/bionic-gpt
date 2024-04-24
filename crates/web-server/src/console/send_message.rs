use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::authz;
use db::queries::chats;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
    pub conversation_id: i64,
    pub prompt_id: i32,
}

pub async fn send_message(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(message): Form<Message>,
) -> Result<impl IntoResponse, CustomError> {
    if message.validate().is_ok() {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        let _permissions =
            authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

        // Store the prompt, ready for the front end webcomponent to pickup
        chats::new_chat()
            .bind(
                &transaction,
                &message.conversation_id,
                &message.prompt_id,
                &message.message,
                &"",
            )
            .await?;

        transaction.commit().await?;

        super::super::layout::redirect(&web_pages::routes::console::conversation_route(
            team_id,
            message.conversation_id,
        ))
    } else {
        super::super::layout::redirect(&web_pages::routes::console::index_route(team_id))
    }
}
