use axum::Router;
use axum_extra::routing::RouterExt;

use axum::extract::Extension;
use axum::response::Html;
use web_pages::{pipelines, routes::document_pipelines::Index};

use crate::{CustomError, Jwt};
use axum::extract::Form;
use axum::response::IntoResponse;
use db::authz;
use db::queries;
use db::queries::document_pipelines;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::document_pipelines::Delete;
use web_pages::routes::document_pipelines::New;

use rand::{distr::Alphanumeric, rng, Rng};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(new_action)
        .typed_post(delete_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_use_api_keys() {
        return Err(CustomError::Authorization);
    }

    let pipelines = queries::document_pipelines::document_pipelines()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let html = pipelines::index::page(team_id, rbac, pipelines, datasets);

    Ok(Html(html))
}

pub async fn delete_action(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_use_api_keys() {
        return Err(CustomError::Authorization);
    }

    queries::document_pipelines::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::document_pipelines::Index { team_id }.to_string(),
        "Document Deleted",
    )
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewForm {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub dataset_id: i32,
}

pub async fn new_action(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(new_pipeline): Form<NewForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_use_api_keys() {
        return Err(CustomError::Authorization);
    }

    if new_pipeline.validate().is_ok() {
        let api_key: String = rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        document_pipelines::insert()
            .bind(
                &transaction,
                &new_pipeline.dataset_id,
                &rbac.user_id,
                &team_id,
                &new_pipeline.name,
                &api_key,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&Index { team_id }.to_string(), "Pipeline Created")
}
