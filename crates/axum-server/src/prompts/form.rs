use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use db::authz;
use db::Pool;
use db::{queries, Transaction};
use serde::Deserialize;
use ui_pages::{prompts::string_to_dataset_connection, string_to_visibility};
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewPromptTemplate {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub system_prompt: String,
    pub dataset_connection: String,
    pub model_id: i32,
    pub datasets: Option<String>,
    pub max_history_items: i32,
    pub max_chunks: i32,
    pub max_tokens: i32,
    pub trim_ratio: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub visibility: String,
}

pub async fn upsert(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(new_prompt_template): Form<NewPromptTemplate>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let visibility = string_to_visibility(&new_prompt_template.visibility);

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
                    &new_prompt_template.name,
                    &visibility,
                    &string_to_dataset_connection(&new_prompt_template.dataset_connection),
                    &system_prompt,
                    &new_prompt_template.max_history_items,
                    &new_prompt_template.max_chunks,
                    &new_prompt_template.max_tokens,
                    &new_prompt_template.trim_ratio,
                    &new_prompt_template.temperature,
                    &new_prompt_template.top_p,
                    &id,
                )
                .await?;

            queries::prompts::delete_prompt_datasets()
                .bind(&transaction, &id)
                .await?;

            insert_datasets(&transaction, id, new_prompt_template.datasets).await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_pages::routes::prompts::index_route(team_id),
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
                    &new_prompt_template.name,
                    &visibility,
                    &string_to_dataset_connection(&new_prompt_template.dataset_connection),
                    &system_prompt,
                    &new_prompt_template.max_history_items,
                    &new_prompt_template.max_chunks,
                    &new_prompt_template.max_tokens,
                    &new_prompt_template.trim_ratio,
                    &new_prompt_template.temperature,
                    &new_prompt_template.top_p,
                )
                .one()
                .await?;

            // Create the connections to any datasets
            insert_datasets(&transaction, prompt_id, new_prompt_template.datasets).await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_pages::routes::prompts::index_route(team_id),
                "Prompt Template Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &ui_pages::routes::prompts::index_route(team_id),
            "Problem with Prompt Validation",
        )
        .into_response()),
    }
}

async fn insert_datasets(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    datasets: Option<String>,
) -> Result<(), CustomError> {
    // Create the connections to any datasets
    if let Some(datasets) = datasets {
        // The environments we have selected for the ser come in as a comma
        // separated list of ids.
        let datasets: Vec<i32> = datasets
            .split(',')
            .map(|e| e.parse::<i32>().unwrap_or(-1))
            .filter(|e| *e != -1)
            .collect();

        for dataset in datasets {
            queries::prompts::insert_prompt_dataset()
                .bind(transaction, &prompt_id, &dataset)
                .await?;
        }
    }

    Ok(())
}
