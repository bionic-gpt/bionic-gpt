use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::{authz, queries, OpenapiSpecCategory, Pool};
use web_pages::openapi_specs::upsert::OpenapiSpecForm;
use web_pages::routes::openapi_specs::{Edit, Index, New};

fn category_to_string(category: OpenapiSpecCategory) -> String {
    match category {
        OpenapiSpecCategory::WebSearch => "WebSearch".to_string(),
        OpenapiSpecCategory::CodeSandbox => "CodeSandbox".to_string(),
        OpenapiSpecCategory::Application => "Application".to_string(),
    }
}

pub async fn index_loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let specs = queries::openapi_specs::list()
        .bind(&transaction)
        .all()
        .await?;

    let html = web_pages::openapi_specs::page::page(team_id, rbac, specs);
    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let form = OpenapiSpecForm::default();
    let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
    Ok(Html(html))
}

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let spec = queries::openapi_specs::by_id()
        .bind(&transaction, &id)
        .one()
        .await?;

    let spec_json =
        serde_json::to_string_pretty(&spec.spec).unwrap_or_else(|_| spec.spec.to_string());

    let form = OpenapiSpecForm {
        id: Some(spec.id),
        slug: spec.slug,
        title: spec.title,
        description: spec.description.unwrap_or_default(),
        logo_url: spec.logo_url.unwrap_or_default(),
        category: category_to_string(spec.category),
        spec: spec_json,
        is_active: spec.is_active,
        error: None,
    };

    let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
    Ok(Html(html))
}
