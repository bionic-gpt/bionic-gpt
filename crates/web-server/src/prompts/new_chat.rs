use super::super::{Authentication, CustomError};
use axum::{extract::Extension, response::IntoResponse};
use db::authz;
use db::queries::conversations;
use db::Pool;
use web_pages::routes::prompts::NewChat;

pub async fn new_chat(
    NewChat { team_id, prompt_id }: NewChat,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let conversation_id = conversations::create_conversation()
        .bind(&transaction, &team_id, &Some(prompt_id))
        .one()
        .await?;

    transaction.commit().await?;

    super::super::layout::redirect(
        &web_pages::routes::console::Conversation {
            team_id,
            conversation_id,
        }
        .to_string(),
    )
}
