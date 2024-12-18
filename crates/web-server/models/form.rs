use super::super::layout::empty_string_is_none;
use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Form;
use db::Pool;
use db::{authz, Visibility};
use db::{queries, ModelType};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::models::New;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ModelForm {
    pub id: Option<i32>,
    pub prompt_id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The display name is mandatory"))]
    pub display_name: String,
    #[validate(length(min = 1, message = "The prompt is mandatory"))]
    pub base_url: String,
    pub model_type: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub api_key: Option<String>,
    pub tpm_limit: i32,
    pub rpm_limit: i32,
    pub context_size: i32,
    pub disclaimer: String,
    pub description: String,
    pub example1: String,
    pub example2: String,
    pub example3: String,
    pub example4: String,
}

pub async fn upsert(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(model_form): Form<ModelForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let model_type = match model_form.model_type.as_str() {
        "LLM" => ModelType::LLM,
        "Image" => ModelType::Image,
        "TextToSpeech" => ModelType::TextToSpeech,
        _ => ModelType::Embeddings,
    };

    match (model_form.validate(), model_form.id) {
        (Ok(_), Some(model_id)) => {
            // The form is valid save to the database
            queries::models::update()
                .bind(
                    &transaction,
                    &model_form.name,
                    &model_type,
                    &model_form.base_url,
                    &model_form.api_key,
                    &model_form.tpm_limit,
                    &model_form.rpm_limit,
                    &model_form.context_size,
                    &model_id,
                )
                .await?;

            let system_prompt: Option<&String> = None;

            if let Some(prompt_id) = model_form.prompt_id {
                queries::prompts::update()
                    .bind(
                        &transaction,
                        &model_id,
                        &0, // Set category to uncategorized
                        &model_form.display_name,
                        &db::Visibility::Company,
                        &system_prompt,
                        &3,
                        &10,
                        &model_form.context_size,
                        &80,
                        &0.7,
                        &model_form.description,
                        &model_form.disclaimer,
                        &Some(&model_form.example1),
                        &Some(&model_form.example2),
                        &Some(&model_form.example3),
                        &Some(&model_form.example4),
                        &db::PromptType::Model,
                        &prompt_id,
                    )
                    .await?;
            }

            transaction.commit().await?;

            Ok(super::super::layout::redirect_and_snackbar(
                &web_pages::routes::models::Index { team_id }.to_string(),
                "Model Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            // The form is valid save to the database
            let model_id = queries::models::insert()
                .bind(
                    &transaction,
                    &model_form.name,
                    &model_type,
                    &model_form.base_url,
                    &model_form.api_key,
                    &model_form.tpm_limit,
                    &model_form.rpm_limit,
                    &model_form.context_size,
                )
                .one()
                .await?;

            let system_prompt: Option<String> = None;
            let image_icon: Option<i32> = None;

            let context_size = if model_form.context_size != 0 {
                model_form.context_size / 2
            } else {
                0
            };

            if model_type == ModelType::LLM {
                queries::prompts::insert()
                    .bind(
                        &transaction,
                        &team_id,
                        &model_id,
                        &0, // Set category to uncategorized
                        &model_form.display_name,
                        &image_icon,
                        &Visibility::Company,
                        &system_prompt,
                        &3,
                        &10,
                        &context_size,
                        &80,
                        &0.7,
                        &model_form.description,
                        &model_form.disclaimer,
                        &Some(&model_form.example1),
                        &Some(&model_form.example2),
                        &Some(&model_form.example3),
                        &Some(&model_form.example4),
                        &db::PromptType::Model,
                    )
                    .one()
                    .await?;
            }

            transaction.commit().await?;

            Ok(super::super::layout::redirect_and_snackbar(
                &web_pages::routes::models::Index { team_id }.to_string(),
                "Model Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(super::super::layout::redirect_and_snackbar(
            &web_pages::routes::models::Index { team_id }.to_string(),
            "Problem with Model Validation",
        )
        .into_response()),
    }
}
