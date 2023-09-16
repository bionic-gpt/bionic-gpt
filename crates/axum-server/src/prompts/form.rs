use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::{Html, IntoResponse};
use axum_extra::extract::Form;
use db::queries;
use db::types::public::DatasetConnection;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

pub async fn new(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;
    let datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models().bind(&transaction).all().await?;
    Ok(Html(ui_components::prompts::form::form(
        team_id, None, datasets, models,
    )))
}

pub async fn edit(
    Path((team_id, prompt_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models().bind(&transaction).all().await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id)
        .one()
        .await?;

    Ok(Html(ui_components::prompts::form::form(
        team_id,
        Some(prompt),
        datasets,
        models,
    )))
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewPromptTemplate {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The prompt is mandatory"))]
    pub template: String,
    pub model_id: i32,
    pub datasets: Vec<String>,
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

    match (new_prompt_template.validate(), new_prompt_template.id) {
        (Ok(_), Some(id)) => {
            // The form is valid save to the database
            queries::prompts::update()
                .bind(
                    &transaction,
                    &team_id,
                    &new_prompt_template.name,
                    &DatasetConnection::None,
                    &new_prompt_template.template,
                    &id,
                )
                .await?;

            queries::prompts::delete_prompt_datasets()
                .bind(&transaction, &id)
                .await?;

            // Create the connections to any datasets
            for dataset in new_prompt_template.datasets {
                dbg!(&dataset);
                queries::prompts::insert_prompt_dataset()
                    .bind(&transaction, &id, &dataset.parse::<i32>().unwrap())
                    .await?;
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
                    &new_prompt_template.name,
                    &DatasetConnection::None,
                    &new_prompt_template.template,
                )
                .one()
                .await?;

            // Create the connections to any datasets
            for dataset in new_prompt_template.datasets {
                queries::prompts::insert_prompt_dataset()
                    .bind(&transaction, &prompt_id, &dataset.parse::<i32>().unwrap())
                    .await?;
            }

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_components::routes::prompts::index_route(team_id),
                "Prompt Template Created",
            )
            .into_response())
        }
        (Err(_), _) => {
            let datasets = queries::datasets::datasets()
                .bind(&transaction)
                .all()
                .await?;

            let models = queries::models::models().bind(&transaction).all().await?;
            let html = ui_components::prompts::form::form(team_id, None, datasets, models);
            Ok(html.into_response())
        }
    }
}
