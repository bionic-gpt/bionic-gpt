use crate::{CustomError, Jwt};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::{authz, queries, OpenapiSpecCategory, Pool};
use web_pages::routes::code_sandbox::{Index, Select};

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_post(select_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

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

    let html = web_pages::code_sandbox::page::page(team_id, rbac, specs, selected_spec_id);

    Ok(Html(html))
}

pub async fn select_action(
    Select { team_id, id }: Select,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

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
