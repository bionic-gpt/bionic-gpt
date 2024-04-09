use super::super::{Authentication, CustomError};
use super::super::layout::empty_string_is_none;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Form;
use db::authz;
use db::Pool;
use db::{queries, ModelType};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ModelForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The prompt is mandatory"))]
    pub base_url: String,
    pub model_type: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub api_key: Option<String>,
    pub billion_parameters: i32,
    pub context_size: i32,
}

pub async fn upsert(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(model_form): Form<ModelForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let model_type = if model_form.model_type == "LLM" {
        ModelType::LLM
    } else {
        ModelType::Embeddings
    };

    match (model_form.validate(), model_form.id) {
        (Ok(_), Some(id)) => {
            // The form is valid save to the database
            queries::models::update()
                .bind(
                    &transaction,
                    &model_form.name,
                    &model_type,
                    &model_form.base_url,
                    &model_form.api_key,
                    &model_form.billion_parameters,
                    &model_form.context_size,
                    &id,
                )
                .await?;

            transaction.commit().await?;

            Ok(super::super::layout::redirect_and_snackbar(
                &ui_pages::routes::models::index_route(team_id),
                "Model Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            // The form is valid save to the database
            queries::models::insert()
                .bind(
                    &transaction,
                    &model_form.name,
                    &model_type,
                    &model_form.base_url,
                    &model_form.api_key,
                    &model_form.billion_parameters,
                    &model_form.context_size,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(super::super::layout::redirect_and_snackbar(
                &ui_pages::routes::models::index_route(team_id),
                "Model Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(super::super::layout::redirect_and_snackbar(
            &ui_pages::routes::models::index_route(team_id),
            "Problem with Model Validation",
        )
        .into_response()),
    }
}
