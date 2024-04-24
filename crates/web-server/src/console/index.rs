use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::IntoResponse;
use db::authz;
use db::queries::conversations;
use db::{Conversation, Pool};
use web_pages::routes::console::Index;

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Get the latest conversation
    let conversation: Result<Conversation, db::TokioPostgresError> =
        conversations::get_latest_conversation()
            .bind(&transaction)
            .one()
            .await;

    let conversation_id = if let Ok(conversation) = conversation {
        conversation.id
    } else {
        conversations::create_conversation()
            .bind(&transaction, &team_id)
            .one()
            .await?
    };

    transaction.commit().await?;

    super::super::layout::redirect(
        &web_pages::routes::console::Conversation {
            team_id,
            conversation_id,
        }
        .to_string(),
    )
}
