// Consolidated documents.rs

use axum::{
    extract::{Extension, Form, Multipart, Query},
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::authz;
use db::queries::{self, datasets, documents, models};
use db::{ModelType, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::documents::{Delete, Index, Processing, Upload};

use crate::{locale::Locale, CustomError, Jwt};

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
    locale: Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let documents = documents::documents()
        .bind(&transaction, &dataset_id)
        .all()
        .await?;

    let dataset = datasets::dataset()
        .bind(&transaction, &dataset_id)
        .one()
        .await?;

    let available_models = models::models()
        .bind(&transaction, &ModelType::Embeddings)
        .all()
        .await?;

    let can_set_visibility_to_company = rbac.is_sys_admin;

    let i18n = db::i18n::global();
    i18n.ensure_locale("en").await;
    if locale.as_str() != "en" {
        i18n.ensure_locale(locale.as_str()).await;
    }

    let html = web_pages::documents::page::page(
        rbac,
        team_id,
        dataset,
        documents,
        available_models,
        can_set_visibility_to_company,
        locale.as_str(),
    );

    Ok(Html(html))
}

// Delete function
#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteDoc {
    pub team_id: String,
    pub document_id: i32,
    pub dataset_id: i32,
    pub project_id: Option<i32>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ProjectDocumentContext {
    pub project_id: Option<i32>,
}

fn document_redirect(team_id: String, dataset_id: i32, project_id: Option<i32>) -> String {
    if let Some(project_id) = project_id {
        web_pages::routes::projects::View {
            team_id,
            project_id,
        }
        .to_string()
    } else {
        web_pages::routes::documents::Index {
            team_id,
            dataset_id,
        }
        .to_string()
    }
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
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &delete_doc.team_id).await?;

    if !rbac.can_manage_projects() && delete_doc.project_id.is_some() {
        return Err(CustomError::Authorization);
    }

    if let Some(project_id) = delete_doc.project_id {
        let project = queries::projects::project()
            .bind(&transaction, &project_id)
            .one()
            .await?;
        if project.dataset_id != delete_doc.dataset_id {
            return Err(CustomError::Authorization);
        }
    }

    queries::documents::delete()
        .bind(&transaction, &delete_doc.document_id)
        .await?;

    transaction.commit().await?;

    let redirect = document_redirect(
        delete_doc.team_id,
        delete_doc.dataset_id,
        delete_doc.project_id,
    );

    crate::layout::redirect_and_snackbar(&redirect, "Document Deleted")
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

    let (_rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

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
    Extension(storage_config): Extension<object_storage::StorageConfig>,
    Query(project_context): Query<ProjectDocumentContext>,
    mut files: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.can_manage_projects() && project_context.project_id.is_some() {
        return Err(CustomError::Authorization);
    }

    if let Some(project_id) = project_context.project_id {
        let project = queries::projects::project()
            .bind(&transaction, &project_id)
            .one()
            .await?;
        if project.dataset_id != dataset_id {
            return Err(CustomError::Authorization);
        }
    }

    while let Some(file) = files.next_field().await.unwrap() {
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap().to_vec();

        let object_id =
            object_storage::upload(&storage_config, rbac.user_id, team_id_num, &name, &data)
                .await
                .map_err(|error| {
                    tracing::error!(
                        error = %error,
                        team_id,
                        dataset_id,
                        file_name = %name,
                        file_size = data.len(),
                        "Failed to upload document to object storage"
                    );
                    CustomError::Database(error.to_string())
                })?;

        let _document_id = queries::documents::insert_with_object()
            .bind(
                &transaction,
                &dataset_id,
                &name,
                &(data.len() as i32),
                &object_id,
            )
            .one()
            .await
            .map_err(|error| {
                tracing::error!(
                    error = %error,
                    team_id,
                    dataset_id,
                    file_name = %name,
                    file_size = data.len(),
                    object_id,
                    "Failed to insert document metadata"
                );
                CustomError::Database(error.to_string())
            })?;
    }

    transaction.commit().await?;

    let redirect = document_redirect(team_id, dataset_id, project_context.project_id);

    crate::layout::redirect_and_snackbar(&redirect, "Document Uploaded")
}

#[cfg(test)]
mod tests {
    use super::document_redirect;

    #[test]
    fn document_redirect_returns_to_project_when_context_is_present() {
        assert_eq!(
            document_redirect("team".to_string(), 12, Some(7)),
            "/o/team/projects/view/7"
        );
    }

    #[test]
    fn document_redirect_preserves_dataset_flow_without_project_context() {
        assert_eq!(
            document_redirect("team".to_string(), 12, None),
            "/o/team/dataset/12/documents"
        );
    }
}
