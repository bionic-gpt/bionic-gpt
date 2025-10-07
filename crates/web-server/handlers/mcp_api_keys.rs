use crate::{CustomError, Jwt};
use axum::response::{Html, IntoResponse};
use axum::{extract::Extension, Router};
use axum_extra::{extract::Form, routing::RouterExt};
use db::{authz, queries, Pool};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde::Deserialize;
use validator::Validate;
use web_pages::mcp_api_keys::{GeneratedKey, NewKeyForm};
use web_pages::routes::mcp_api_keys::{Create, Delete, Index};

async fn load_page_data(
    transaction: &db::Transaction<'_>,
    team_id: i32,
) -> Result<Vec<db::ApiKey>, CustomError> {
    let keys = queries::api_keys::find_mcp_api_keys()
        .bind(transaction, &team_id)
        .all()
        .await?;

    Ok(keys)
}

#[derive(Deserialize, Validate, Debug)]
pub struct CreateMcpApiKeyForm {
    #[validate(length(min = 1, message = "Please provide a name"))]
    pub name: String,
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_mcp_keys() {
        return Err(CustomError::Authorization);
    }

    let keys = load_page_data(&transaction, team_id).await?;

    let html = web_pages::mcp_api_keys::page(rbac, team_id, keys, NewKeyForm::default(), None);

    Ok(Html(html))
}

pub async fn create_action(
    Create { team_id }: Create,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<CreateMcpApiKeyForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_mcp_keys() {
        return Err(CustomError::Authorization);
    }

    if form.validate().is_err() {
        let keys = load_page_data(&transaction, team_id).await?;
        let html = web_pages::mcp_api_keys::page(
            rbac,
            team_id,
            keys,
            NewKeyForm {
                name: form.name.clone(),
                error: Some("Please correct the highlighted fields".to_string()),
            },
            None,
        );

        return Ok(Html(html).into_response());
    }

    let api_key: String = rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(char::from)
        .collect();

    queries::api_keys::new_mcp_api_key()
        .bind(&transaction, &rbac.user_id, &team_id, &form.name, &api_key)
        .one()
        .await?;

    let keys = load_page_data(&transaction, team_id).await?;

    transaction.commit().await?;

    let html = web_pages::mcp_api_keys::page(
        rbac,
        team_id,
        keys,
        NewKeyForm::default(),
        Some(GeneratedKey {
            name: form.name,
            value: api_key,
        }),
    );

    Ok(Html(html).into_response())
}

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(create_action)
        .typed_post(delete_action)
}

pub async fn delete_action(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_mcp_keys() {
        return Err(CustomError::Authorization);
    }

    queries::api_keys::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::mcp_api_keys::Index { team_id }.to_string(),
        "API Key Deleted",
    )
}
