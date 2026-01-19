use crate::layout::empty_string_is_none;
use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::providers::upsert as provider_page;
use web_pages::routes::providers::{Delete, Edit, Index, New, Upsert};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_get(new_loader)
        .typed_get(edit_loader)
        .typed_post(upsert_action)
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

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let providers = queries::providers::providers()
        .bind(&transaction)
        .all()
        .await?;

    let html = web_pages::providers::page::page(team_id, rbac, providers);

    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let form = provider_page::ProviderForm {
        id: None,
        name: "".to_string(),
        svg_logo: "".to_string(),
        default_model_name: "".to_string(),
        default_model_display_name: "".to_string(),
        default_model_context_size: 0,
        default_model_description: "".to_string(),
        base_url: "".to_string(),
        error: None,
    };

    let html = provider_page::page(team_id, rbac, form);

    Ok(Html(html))
}

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let provider = queries::providers::provider()
        .bind(&transaction, &id)
        .one()
        .await?;

    let form = provider_page::ProviderForm {
        id: Some(provider.id),
        name: provider.name,
        svg_logo: provider.svg_logo,
        default_model_name: provider.default_model_name.unwrap_or_default(),
        default_model_display_name: provider.default_model_display_name.unwrap_or_default(),
        default_model_context_size: provider.default_model_context_size,
        default_model_description: provider.default_model_description,
        base_url: provider.base_url,
        error: None,
    };

    let html = provider_page::page(team_id, rbac, form);

    Ok(Html(html))
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ProviderForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The SVG logo is mandatory"))]
    pub svg_logo: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub default_model_name: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub default_model_display_name: Option<String>,
    pub default_model_context_size: i32,
    #[validate(length(min = 1, message = "The default model description is mandatory"))]
    pub default_model_description: String,
    #[validate(length(min = 1, message = "The base URL is mandatory"))]
    pub base_url: String,
}

pub async fn upsert_action(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<ProviderForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    match (form.validate(), form.id) {
        (Ok(_), Some(id)) => {
            queries::providers::update()
                .bind(
                    &transaction,
                    &form.name,
                    &form.svg_logo,
                    &form.default_model_name,
                    &form.default_model_display_name,
                    &form.default_model_context_size,
                    &form.default_model_description,
                    &form.base_url,
                    &id,
                )
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::providers::Index { team_id }.to_string(),
                "Provider Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            queries::providers::insert()
                .bind(
                    &transaction,
                    &form.name,
                    &form.svg_logo,
                    &form.default_model_name,
                    &form.default_model_display_name,
                    &form.default_model_context_size,
                    &form.default_model_description,
                    &form.base_url,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::providers::Index { team_id }.to_string(),
                "Provider Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::providers::Index { team_id }.to_string(),
            "Problem with Provider Validation",
        )
        .into_response()),
    }
}

pub async fn delete_action(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    queries::providers::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::providers::Index { team_id }.to_string(),
        "Provider Deleted",
    )
}
