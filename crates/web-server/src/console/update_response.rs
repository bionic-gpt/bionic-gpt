use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::Pool;
use db::{authz, PromptType};
use db::{
    queries::{chats, models, prompts},
    ChatStatus,
};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::console::UpdateResponse;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Chat {
    pub chat_id: i32,
    pub response: String,
}

/// When the front end has finished streaming the response from the model
/// it will submit a form that directs to here. The response has already
/// been saved in the database so here we can redirect to the conversation.
///
/// Embeddings - At this point we have the complete response so we can generate
/// embeddings for the response that are used by the search feature.
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

    // If the streaming LLM proxy saved a response, we'll process it.
    if let Some(response) = chat.response {
        // We generate embeddings so we can search the history.
        let embeddings_model = models::get_system_embedding_model()
            .bind(&transaction)
            .one()
            .await?;

        let embeddings = embeddings_api::get_embeddings(
            &response,
            &embeddings_model.base_url,
            &embeddings_model.name,
            &embeddings_model.api_key,
        )
        .await
        .map_err(|e| CustomError::ExternalApi(e.to_string()));

        match embeddings {
            Ok(embeddings) => {
                let embedding_data = pgvector::Vector::from(embeddings);
                transaction
                    .execute(
                        "
                        UPDATE chats SET response_embeddings = $1
                        WHERE id = $2
                        ",
                        &[&embedding_data, &chat.id],
                    )
                    .await?;
            }
            Err(e) => {
                tracing::error!("Problem trying to get embeddings data {}", e);
            }
        }
    } else {
        // There's no response from the stremaing proxy. Can we use what
        // came form the front end?
        chats::update_chat()
            .bind(
                &transaction,
                &message.response,
                &100,
                &ChatStatus::Error,
                &message.chat_id,
            )
            .await?;
    }

    let prompt = prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &team_id)
        .one()
        .await?;

    transaction.commit().await?;

    if prompt.prompt_type == PromptType::Assistant {
        super::super::layout::redirect(
            &web_pages::routes::prompts::Conversation {
                team_id,
                conversation_id: chat.conversation_id,
                prompt_id: prompt.id,
            }
            .to_string(),
        )
    } else {
        super::super::layout::redirect(
            &web_pages::routes::console::Conversation {
                team_id,
                conversation_id: chat.conversation_id,
            }
            .to_string(),
        )
    }
}
