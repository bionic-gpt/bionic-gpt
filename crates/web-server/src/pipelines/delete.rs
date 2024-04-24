use super::super::{Authentication, CustomError};
use axum::{extract::Extension, response::IntoResponse};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::document_pipelines::Delete;

pub async fn delete(
    Delete { id, team_id }: Delete,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::document_pipelines::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(
        &web_pages::routes::document_pipelines::Index { team_id }.to_string(),
        "Document Deleted",
    )
}
