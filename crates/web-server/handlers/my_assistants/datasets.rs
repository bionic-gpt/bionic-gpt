use crate::{CustomError, Jwt};
use axum::response::Html;
use axum::{extract::Extension, response::IntoResponse};
use axum_extra::extract::Form;
use db::{authz, queries, Pool, Transaction};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    my_assistants,
    routes::prompts::{ManageDatasets, UpdateDatasets},
};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DatasetUpdateForm {
    #[serde(default)]
    pub datasets: Vec<i32>,
}

async fn update_datasets(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    datasets: Vec<i32>,
) -> Result<(), CustomError> {
    for dataset in datasets {
        queries::prompts::insert_prompt_dataset()
            .bind(transaction, &prompt_id, &dataset)
            .await?;
    }
    Ok(())
}

pub async fn update_datasets_action(
    UpdateDatasets { team_id, prompt_id }: UpdateDatasets,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<DatasetUpdateForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Delete existing dataset connections
    queries::prompts::delete_prompt_datasets()
        .bind(&transaction, &prompt_id)
        .await?;

    // Add new dataset connections
    update_datasets(&transaction, prompt_id, form.datasets).await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::prompts::View { team_id, prompt_id }.to_string(),
        "Dataset connections updated successfully",
    )
    .into_response())
}

pub async fn manage_datasets(
    ManageDatasets { team_id, prompt_id }: ManageDatasets,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    // Parse selected dataset IDs from comma-separated string
    let selected_dataset_ids: Vec<i32> = if prompt.selected_datasets.is_empty() {
        Vec::new()
    } else {
        prompt
            .selected_datasets
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    let form = my_assistants::datasets::DatasetForm {
        prompt_id: prompt.id,
        prompt_name: prompt.name,
        datasets,
        selected_dataset_ids,
        error: None,
    };

    let html = my_assistants::datasets::page(team_id, rbac, form);

    Ok(Html(html))
}
