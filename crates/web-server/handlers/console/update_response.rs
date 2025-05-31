use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::queries::{chats, prompts};
use db::Pool;
use db::{authz, PromptType};
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

    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let chat = chats::chat()
        .bind(&transaction, &message.chat_id)
        .one()
        .await?;

    let prompt = prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &team_id)
        .one()
        .await?;

    transaction.commit().await?;

    tracing::debug!("DB Transaction committed");

    if prompt.prompt_type == PromptType::Assistant {
        crate::layout::redirect(
            &web_pages::routes::prompts::Conversation {
                team_id,
                conversation_id: chat.conversation_id,
                prompt_id: prompt.id,
            }
            .to_string(),
        )
    } else {
        crate::layout::redirect(
            &web_pages::routes::console::Conversation {
                team_id,
                conversation_id: chat.conversation_id,
            }
            .to_string(),
        )
    }
}
