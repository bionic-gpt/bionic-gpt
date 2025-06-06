use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum_extra::extract::Form;
use axum_extra::routing::RouterExt;
use db::authz;
use db::{queries, Pool};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::api_keys::{Delete, New};
use web_pages::{api_keys, routes::api_keys::Index};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(new_api_key_action)
        .typed_post(delete_api_key_action)
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

    let api_keys = queries::api_keys::api_keys()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let assistants = queries::prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Assistant)
        .all()
        .await?;

    let models = queries::prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Model)
        .all()
        .await?;

    // Fetch graph data for the last 7 days
    let token_usage_data = queries::token_usage_metrics::get_daily_token_usage_for_team()
        .bind(&transaction, &team_id, &"7")
        .all()
        .await?;

    let api_request_data = queries::token_usage_metrics::get_daily_api_request_count_for_team()
        .bind(&transaction, &team_id, &"7")
        .all()
        .await?;

    let html = api_keys::index::page(
        rbac,
        team_id,
        api_keys,
        assistants,
        models,
        token_usage_data,
        api_request_data,
    );

    Ok(Html(html))
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewApiKey {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub prompt_id: i32,
}

pub async fn new_api_key_action(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(new_api_key): Form<NewApiKey>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if new_api_key.validate().is_ok() {
        let api_key: String = rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        queries::api_keys::new_api_key()
            .bind(
                &transaction,
                &new_api_key.prompt_id,
                &rbac.user_id,
                &team_id,
                &new_api_key.name,
                &api_key,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&Index { team_id }.to_string(), "Api Key Created")
}

pub async fn delete_api_key_action(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::api_keys::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::api_keys::Index { team_id }.to_string(),
        "Document Deleted",
    )
}
