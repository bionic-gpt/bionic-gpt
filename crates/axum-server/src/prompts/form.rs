use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use db::types::public::DatasetConnection;
use db::Pool;
use db::{queries, Visibility};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewPromptTemplate {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The prompt is mandatory"))]
    pub template: String,
    pub dataset_connection: String,
    pub model_id: i32,
    pub datasets: Option<Vec<String>>,
    pub min_history_items: i32,
    pub max_history_items: i32,
    pub min_chunks: i32,
    pub max_chunks: i32,
    pub max_tokens: i32,
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
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let visibility = if new_prompt_template.visibility == "Private" {
        Visibility::Private
    } else {
        Visibility::Team
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
                    &dataset_connection_from_string(&new_prompt_template.dataset_connection),
                    &new_prompt_template.template,
                    &new_prompt_template.min_history_items,
                    &new_prompt_template.max_history_items,
                    &new_prompt_template.min_chunks,
                    &new_prompt_template.max_chunks,
                    &new_prompt_template.max_tokens,
                    &new_prompt_template.temperature,
                    &new_prompt_template.top_p,
                    &id,
                )
                .await?;

            queries::prompts::delete_prompt_datasets()
                .bind(&transaction, &id)
                .await?;

            // Create the connections to any datasets
            if let Some(datasets) = new_prompt_template.datasets {
                for dataset in datasets {
                    queries::prompts::insert_prompt_dataset()
                        .bind(&transaction, &id, &dataset.parse::<i32>().unwrap())
                        .await?;
                }
            }

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_components::routes::prompts::index_route(team_id),
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
                    &dataset_connection_from_string(&new_prompt_template.dataset_connection),
                    &new_prompt_template.template,
                    &new_prompt_template.min_history_items,
                    &new_prompt_template.max_history_items,
                    &new_prompt_template.min_chunks,
                    &new_prompt_template.max_chunks,
                    &new_prompt_template.max_tokens,
                    &new_prompt_template.temperature,
                    &new_prompt_template.top_p,
                )
                .one()
                .await?;

            // Create the connections to any datasets
            if let Some(datasets) = new_prompt_template.datasets {
                for dataset in datasets {
                    queries::prompts::insert_prompt_dataset()
                        .bind(&transaction, &prompt_id, &dataset.parse::<i32>().unwrap())
                        .await?;
                }
            }

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_components::routes::prompts::index_route(team_id),
                "Prompt Template Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &ui_components::routes::prompts::index_route(team_id),
            "Problem with Prompt Validation",
        )
        .into_response()),
    }
}

fn dataset_connection_from_string(dataset_connection: &str) -> DatasetConnection {
    match dataset_connection {
        "All" => DatasetConnection::All,
        "None" => DatasetConnection::None,
        _ => DatasetConnection::Selected,
    }
}
