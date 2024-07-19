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
use web_pages::routes::console::SendMessage;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
    pub conversation_id: i64,
    pub prompt_id: i32,
}

pub async fn send_message(
    SendMessage { team_id }: SendMessage,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(message): Form<Message>,
) -> Result<impl IntoResponse, CustomError> {
    if message.validate().is_ok() {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        let _permissions =
            authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

        // Store the prompt, ready for the front end webcomponent to pickup
        let chat_id = chats::new_chat()
            .bind(
                &transaction,
                &message.conversation_id,
                &message.prompt_id,
                &message.message,
                &"",
            )
            .one()
            .await?;

        // We generate embeddings so we can search the history.
        let embeddings_model = models::get_system_embedding_model()
            .bind(&transaction)
            .one()
            .await?;

        let embeddings = embeddings_api::get_embeddings(
            &message.message,
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
                        UPDATE chats SET request_embeddings = $1
                        WHERE id = $2
                        ",
                        &[&embedding_data, &chat_id],
                    )
                    .await?;
            }
            Err(e) => {
                tracing::error!("Problem trying to get embeddings data {}", e);
            }
        }

        transaction.commit().await?;

        super::super::layout::redirect(
            &web_pages::routes::console::Conversation {
                team_id,
                conversation_id: message.conversation_id,
            }
            .to_string(),
        )
    } else {
        super::super::layout::redirect(&web_pages::routes::console::Index { team_id }.to_string())
    }
}
