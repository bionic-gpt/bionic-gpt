use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse},
};
use db::{authz, queries, Json, OpenapiSpecCategory, Pool};
use validator::Validate;
use web_pages::openapi_specs::upsert::OpenapiSpecForm;
use web_pages::routes::openapi_specs::{Delete, Upsert};

fn parse_category(category: &str) -> OpenapiSpecCategory {
    match category {
        "WebSearch" => OpenapiSpecCategory::WebSearch,
        "CodeSandbox" => OpenapiSpecCategory::CodeSandbox,
        _ => OpenapiSpecCategory::Application,
    }
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(mut form): Form<OpenapiSpecForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    // Trim whitespace from inputs
    form.slug = form.slug.trim().to_string();
    form.title = form.title.trim().to_string();
    form.description = form.description.trim().to_string();
    form.logo_url = form.logo_url.trim().to_string();
    form.category = form.category.trim().to_string();
    form.spec = form.spec.trim().to_string();

    if let Err(validation) = form.validate() {
        form.error = Some(format!("Validation error: {}", validation));
        let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
        return Ok(Html(html).into_response());
    }

    let parsed_spec = match serde_json::from_str::<serde_json::Value>(&form.spec) {
        Ok(value) => value,
        Err(error) => {
            form.error = Some(format!("Invalid JSON: {}", error));
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };

    let description_param = if form.description.is_empty() {
        None
    } else {
        Some(form.description.as_str())
    };

    let logo_url_param = if form.logo_url.is_empty() {
        None
    } else {
        Some(form.logo_url.as_str())
    };

    let category = parse_category(&form.category);
    let spec_json = Json(parsed_spec);

    let result: Result<(), db::TokioPostgresError> = if let Some(id) = form.id {
        queries::openapi_specs::update()
            .bind(
                &transaction,
                &form.slug,
                &form.title,
                &description_param,
                &spec_json,
                &logo_url_param,
                &category,
                &form.is_active,
                &id,
            )
            .await
            .map(|_| ())
    } else {
        queries::openapi_specs::insert()
            .bind(
                &transaction,
                &form.slug,
                &form.title,
                &description_param,
                &spec_json,
                &logo_url_param,
                &category,
                &form.is_active,
            )
            .one()
            .await
            .map(|_| ())
    };

    if let Err(error) = result {
        if let Some(db_error) = error.as_db_error() {
            if db_error.code().code() == "23505" {
                form.error = Some("Slug already exists. Please choose another one.".to_string());
                let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
                return Ok(Html(html).into_response());
            }
        }
        return Err(CustomError::from(error));
    }

    transaction.commit().await?;

    let message = if form.id.is_some() {
        "OpenAPI spec updated"
    } else {
        "OpenAPI spec created"
    };

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        message,
    )
    .into_response())
}

pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    queries::openapi_specs::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        "OpenAPI spec deleted",
    )
    .into_response())
}
