use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::IntoResponse};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::prompts::DeleteConv;

pub async fn delete(
    DeleteConv {
        team_id,
        prompt_id,
        conversation_id,
    }: DeleteConv,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::conversations::delete()
        .bind(&transaction, &conversation_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::prompts::NewChat { team_id, prompt_id }.to_string(),
        "Chat Deleted",
    )
}
