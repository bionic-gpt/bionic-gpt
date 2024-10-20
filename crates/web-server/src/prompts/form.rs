use crate::config::Config;

use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use db::authz;
use db::Pool;
use db::Visibility;
use db::{queries, Transaction};
use serde::Deserialize;
use validator::Validate;
use web_pages::{routes::prompts::Upsert, string_to_visibility};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewPromptTemplate {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub system_prompt: String,
    pub model_id: i32,
    pub category_id: i32,
    #[serde(default)]
    pub datasets: Vec<i32>,
    pub max_history_items: i32,
    pub max_chunks: i32,
    pub max_tokens: i32,
    pub trim_ratio: i32,
    pub temperature: f32,
    pub visibility: String,
    #[validate(length(min = 1, message = "The description is mandatory"))]
    pub description: String,
    pub disclaimer: String,
    pub example1: Option<String>,
    pub example2: Option<String>,
    pub example3: Option<String>,
    pub example4: Option<String>,
}

pub async fn upsert(
    Upsert { team_id }: Upsert,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    Form(new_prompt_template): Form<NewPromptTemplate>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let mut visibility = string_to_visibility(&new_prompt_template.visibility);

    // Is someone trying to override the permitted form values?
    if visibility == Visibility::Company && config.saas {
        visibility = Visibility::Team;
    }

    // Id the system prompt is empty store it as null
    let system_prompt = if new_prompt_template.system_prompt.is_empty() {
        None
    } else {
        Some(&new_prompt_template.system_prompt)
    };

    match (new_prompt_template.validate(), new_prompt_template.id) {
        (Ok(_), Some(id)) => {
            // The form is valid save to the database
            queries::prompts::update()
                .bind(
                    &transaction,
                    &new_prompt_template.model_id,
                    &new_prompt_template.category_id,
                    &new_prompt_template.name,
                    &visibility,
                    &system_prompt,
                    &new_prompt_template.max_history_items,
                    &new_prompt_template.max_chunks,
                    &new_prompt_template.max_tokens,
                    &new_prompt_template.trim_ratio,
                    &new_prompt_template.temperature,
                    &new_prompt_template.description,
                    &new_prompt_template.disclaimer,
                    &new_prompt_template.example1,
                    &new_prompt_template.example2,
                    &new_prompt_template.example3,
                    &new_prompt_template.example4,
                    &db::PromptType::Assistant,
                    &id,
                )
                .await?;

            queries::prompts::delete_prompt_datasets()
                .bind(&transaction, &id)
                .await?;

            insert_datasets(&transaction, id, new_prompt_template.datasets).await?;

            transaction.commit().await?;

            Ok(super::super::layout::redirect_and_snackbar(
                &web_pages::routes::prompts::Index { team_id }.to_string(),
                "Prompt Template Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            // The form is valid save to the database
            let prompt_id = queries::prompts::insert()
                .bind(
                    &transaction,
                    &team_id,
                    &new_prompt_template.model_id,
                    &new_prompt_template.category_id,
                    &new_prompt_template.name,
                    &visibility,
                    &system_prompt,
                    &new_prompt_template.max_history_items,
                    &new_prompt_template.max_chunks,
                    &new_prompt_template.max_tokens,
                    &new_prompt_template.trim_ratio,
                    &new_prompt_template.temperature,
                    &new_prompt_template.description,
                    &new_prompt_template.disclaimer,
                    &new_prompt_template.example1,
                    &new_prompt_template.example2,
                    &new_prompt_template.example3,
                    &new_prompt_template.example4,
                    &db::PromptType::Assistant,
                )
                .one()
                .await?;

            // Create the connections to any datasets
            insert_datasets(&transaction, prompt_id, new_prompt_template.datasets).await?;

            transaction.commit().await?;

            Ok(super::super::layout::redirect_and_snackbar(
                &web_pages::routes::prompts::Index { team_id }.to_string(),
                "Prompt Template Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(super::super::layout::redirect_and_snackbar(
            &web_pages::routes::prompts::Index { team_id }.to_string(),
            "Problem with Prompt Validation",
        )
        .into_response()),
    }
}

async fn insert_datasets(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    datasets: Vec<i32>,
) -> Result<(), CustomError> {
    // Create the connections to any datasets
    for dataset in datasets {
        queries::prompts::insert_prompt_dataset()
            .bind(transaction, &prompt_id, &dataset)
            .await?;
    }

    Ok(())
}
