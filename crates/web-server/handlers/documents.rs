// Consolidated documents.rs

use axum::{
    extract::{Extension, Form, Multipart},
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::authz;
use db::queries::{self, datasets, documents};
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::documents::{Delete, Index, Processing, Upload};

use crate::{CustomError, Jwt};

// Router setup
pub fn routes() -> Router {
    Router::new()
        .typed_post(upload_action)
        .typed_post(delete_action)
        .typed_get(row)
        .layer(axum::extract::DefaultBodyLimit::max(50000000))
        .typed_get(loader)
}

// Index function
pub async fn loader(
    Index {
        team_id,
        dataset_id,
    }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let documents = documents::documents()
        .bind(&transaction, &dataset_id)
        .all()
        .await?;

    let dataset = datasets::dataset()
        .bind(&transaction, &dataset_id)
        .one()
        .await?;

    let html = web_pages::documents::index::page(rbac, team_id, dataset, documents);

    Ok(Html(html))
}

// Delete function
#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteDoc {
    pub team_id: i32,
    pub document_id: i32,
    pub dataset_id: i32,
}

pub async fn delete_action(
    Delete {
        team_id: _,
        document_id: _,
    }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(delete_doc): Form<DeleteDoc>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions =
        authz::get_permissions(&transaction, &current_user.into(), delete_doc.team_id).await?;

    queries::documents::delete()
        .bind(&transaction, &delete_doc.document_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::documents::Index {
            team_id: delete_doc.team_id,
            dataset_id: delete_doc.dataset_id,
        }
        .to_string(),
        "Document Deleted",
    )
}

// Processing function
pub async fn row(
    Processing {
        team_id,
        document_id,
    }: Processing,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let document = documents::document()
        .bind(&transaction, &document_id)
        .one()
        .await?;

    let html = web_pages::documents::status::status(document, team_id, false);

    Ok(Html(html))
}

// Upload function
pub async fn upload_action(
    Upload {
        team_id,
        dataset_id,
    }: Upload,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    mut files: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    while let Some(file) = files.next_field().await.unwrap() {
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap().to_vec();

        let _document_id = queries::documents::insert()
            .bind(
                &transaction,
                &dataset_id,
                &name,
                &data,
                &(data.len() as i32),
            )
            .one()
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::documents::Index {
            team_id,
            dataset_id,
        }
        .to_string(),
        "Document Uploaded",
    )
}
