use crate::{CustomError, Jwt};
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
    pub chat_id: i32,
}

/// When the front end has finished streaming the response from the model
/// it will submit a form that directs to here. The response has already
/// been saved in the database so here we can redirect to the conversation.
///
/// Embeddings - At this point we have the complete response so we can generate
/// embeddings for the response that are used by the search feature.
pub async fn update_response(
    UpdateResponse { team_id }: UpdateResponse,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(message): Form<Chat>,
) -> Result<impl IntoResponse, CustomError> {
    tracing::debug!("Receiving end of stream update from the front end");
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (_permissions, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let chat = chats::chat()
        .bind(&transaction, &message.chat_id)
        .one()
        .await?;

    transaction.commit().await?;

    tracing::debug!("DB Transaction committed");

    crate::layout::redirect(
        &web_pages::routes::console::Conversation {
            team_id,
            conversation_id: chat.conversation_id,
        }
        .to_string(),
    )
}
