use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteDoc {
    pub team_id: i32,
    pub document_id: i32,
    pub dataset_id: i32,
}

pub async fn delete(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(delete_doc): Form<DeleteDoc>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::documents::delete()
        .bind(&transaction, &delete_doc.document_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::documents::index_route(delete_doc.team_id, delete_doc.dataset_id),
        "Document Deleted",
    )
}
