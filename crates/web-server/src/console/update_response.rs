use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries::{chats, models};
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::console::UpdateResponse;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Chat {
    pub response: String,
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
    }

    transaction.commit().await?;

    super::super::layout::redirect(
        &web_pages::routes::console::Conversation {
            team_id,
            conversation_id: chat.conversation_id,
        }
        .to_string(),
    )
}
