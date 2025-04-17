use super::super::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::queries::{chats, conversations, models, prompts};
use db::Pool;
use db::{authz, PromptType};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::console::SendMessage;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
    pub conversation_id: Option<i64>,
    pub prompt_id: i32,
}

pub async fn send_message(
    SendMessage { team_id }: SendMessage,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(message): Form<Message>,
) -> Result<impl IntoResponse, CustomError> {
    if message.validate().is_ok() {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        let _permissions =
            authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

        let conversation_id = if let Some(conversation_id) = message.conversation_id {
            conversation_id
        } else {
            conversations::create_conversation()
                .bind(&transaction, &team_id, &None)
                .one()
                .await?
        };

        // Store the prompt, ready for the front end webcomponent to pickup
        let chat_id = chats::new_chat()
            .bind(
                &transaction,
                &conversation_id,
                &message.prompt_id,
                &message.message,
                &"",
            )
            .one()
            .await?;

        // Handle embeddings
        handle_embeddings(&transaction, &message.message, &chat_id).await?;

        let prompt = prompts::prompt()
            .bind(&transaction, &message.prompt_id, &team_id)
            .one()
            .await?;

        transaction.commit().await?;

        if prompt.prompt_type == PromptType::Assistant {
            super::super::layout::redirect(
                &web_pages::routes::prompts::Conversation {
                    team_id,
                    conversation_id,
                    prompt_id: prompt.id,
                }
                .to_string(),
            )
        } else {
            super::super::layout::redirect(
                &web_pages::routes::console::Conversation {
                    team_id,
                    conversation_id,
                }
                .to_string(),
            )
        }
    } else {
        super::super::layout::redirect(&web_pages::routes::console::Index { team_id }.to_string())
    }
}

/// Handles the generation of embeddings and updates the database.
///
/// # Arguments
///
/// * `transaction` - A reference to the current database transaction.
/// * `message` - The message string to generate embeddings for.
/// * `chat_id` - The ID of the chat to update with the embeddings.
///
/// # Returns
///
/// * `Result<(), CustomError>` - Returns `Ok` if successful, or a `CustomError` otherwise.
async fn handle_embeddings(
    transaction: &db::Transaction<'_>,
    message: &str,
    chat_id: &i32,
) -> Result<(), CustomError> {
    // Fetch the embeddings model
    let embeddings_model = models::get_system_embedding_model()
        .bind(transaction)
        .one()
        .await?;

    // Generate embeddings using the external API
    let embeddings = embeddings_api::get_embeddings(
        message,
        &embeddings_model.base_url,
        &embeddings_model.name,
        embeddings_model.context_size,
        &embeddings_model.api_key,
    )
    .await
    .map_err(|e| CustomError::ExternalApi(e.to_string()))?;

    // Convert embeddings to pgvector and update the database
    let embedding_data = pgvector::Vector::from(embeddings);
    transaction
        .execute(
            "
            UPDATE chats SET request_embeddings = $1
            WHERE id = $2
            ",
            &[&embedding_data, chat_id],
        )
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(())
}
