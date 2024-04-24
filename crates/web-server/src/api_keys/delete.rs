use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::api_keys::Delete;

pub async fn delete(
    Delete { id, team_id } : Delete,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(delete_form): Form<Delete>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions =
        authz::get_permissions(&transaction, &current_user.into(), delete_form.team_id).await?;

    queries::api_keys::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(
        &web_pages::routes::api_keys::Index { team_id: team_id }.to_string(),
        "Document Deleted",
    )
}
