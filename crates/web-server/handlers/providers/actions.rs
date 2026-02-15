use crate::layout::{empty_string_is_none, empty_string_is_none_i32};
use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Form;
use db::authz;
use db::{queries, Pool};
use serde::{Deserialize, Deserializer};
use validator::Validate;
use web_pages::routes::providers::{Delete, Upsert};

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
    #[serde(default, deserialize_with = "checkbox_bool")]
    pub api_key_optional: bool,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub default_embeddings_model_name: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub default_embeddings_model_display_name: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none_i32")]
    pub default_embeddings_model_context_size: Option<i32>,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub default_embeddings_model_description: Option<String>,
}

fn checkbox_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(matches!(
        value.as_deref(),
        Some("on") | Some("true") | Some("1")
    ))
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<ProviderForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

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
                    &form.api_key_optional,
                    &form.default_embeddings_model_name,
                    &form.default_embeddings_model_display_name,
                    &form.default_embeddings_model_context_size,
                    &form.default_embeddings_model_description,
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
                    &form.api_key_optional,
                    &form.default_embeddings_model_name,
                    &form.default_embeddings_model_display_name,
                    &form.default_embeddings_model_context_size,
                    &form.default_embeddings_model_description,
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

pub async fn action_delete(
    Delete { id, team_id }: Delete,
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

    queries::providers::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::providers::Index { team_id }.to_string(),
        "Provider Deleted",
    )
}
