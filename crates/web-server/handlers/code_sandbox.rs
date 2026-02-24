use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::{authz, queries, OpenapiSpecCategory, Pool};
use serde::Deserialize;
use tool_runtime::BionicOpenAPI;
use validator::Validate;
use web_pages::routes::code_sandbox::{ConfigureApiKey, DeleteApiKey, Index, Select};
use web_pages::shared::openapi_spec_api_keys::OpenapiSpecKeySummary;

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(select_action)
        .typed_post(configure_api_key_action)
        .typed_post(delete_api_key_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let specs = queries::openapi_specs::by_category()
        .bind(&transaction, &OpenapiSpecCategory::CodeSandbox)
        .all()
        .await?;

    let selected = queries::openapi_spec_selections::selection()
        .bind(&transaction, &OpenapiSpecCategory::CodeSandbox)
        .opt()
        .await?;

    let selected_spec_id = selected.map(|row| row.openapi_spec_id);

    let mut summaries = Vec::new();
    for spec in specs.into_iter() {
        let has_api_key = BionicOpenAPI::new(&spec.spec)
            .map(|openapi| openapi.has_api_key_security())
            .unwrap_or(false);
        let has_key_configured = if has_api_key {
            queries::openapi_spec_api_keys::status()
                .bind(&transaction, &spec.id)
                .one()
                .await?
                .has_key
        } else {
            false
        };

        summaries.push(OpenapiSpecKeySummary {
            spec,
            has_api_key,
            has_key_configured,
        });
    }

    let html = web_pages::code_sandbox::page::page(team_id, rbac, summaries, selected_spec_id);

    Ok(Html(html))
}

pub async fn select_action(
    Select { team_id, id }: Select,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let spec = queries::openapi_specs::by_id()
        .bind(&transaction, &id)
        .one()
        .await?;

    if spec.category != OpenapiSpecCategory::CodeSandbox || !spec.is_active {
        return Err(CustomError::Authorization);
    }

    queries::openapi_spec_selections::set_selection()
        .bind(&transaction, &OpenapiSpecCategory::CodeSandbox, &spec.id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::code_sandbox::Index { team_id }.to_string(),
        "CodeSandbox spec updated",
    )
    .into_response())
}

#[derive(Deserialize, Validate, Debug)]
pub struct ApiKeyForm {
    #[validate(length(min = 1, message = "API key is required"))]
    pub api_key: String,
}

pub async fn configure_api_key_action(
    ConfigureApiKey { team_id, id }: ConfigureApiKey,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(api_key_form): Form<ApiKeyForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let spec = queries::openapi_specs::by_id()
        .bind(&transaction, &id)
        .one()
        .await?;

    if spec.category != OpenapiSpecCategory::CodeSandbox {
        return Err(CustomError::Authorization);
    }

    if api_key_form.validate().is_err() {
        return Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::code_sandbox::Index { team_id }.to_string(),
            "API key is required",
        )
        .into_response());
    }

    queries::openapi_spec_api_keys::upsert()
        .bind(&transaction, &spec.id, &api_key_form.api_key)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::code_sandbox::Index { team_id }.to_string(),
        "Code Sandbox API key updated",
    )
    .into_response())
}

pub async fn delete_api_key_action(
    DeleteApiKey { team_id, id }: DeleteApiKey,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let spec = queries::openapi_specs::by_id()
        .bind(&transaction, &id)
        .one()
        .await?;

    if spec.category != OpenapiSpecCategory::CodeSandbox {
        return Err(CustomError::Authorization);
    }

    queries::openapi_spec_api_keys::delete()
        .bind(&transaction, &spec.id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::code_sandbox::Index { team_id }.to_string(),
        "Code Sandbox API key deleted",
    )
    .into_response())
}
