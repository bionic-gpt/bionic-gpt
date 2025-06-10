use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::ModelType;
use db::Pool;
use web_pages::visibility_to_string;
use web_pages::{
    my_assistants,
    routes::prompts::{ManageDatasets, ManageIntegrations, MyAssistants, View},
};

pub async fn my_assistants(
    MyAssistants { team_id }: MyAssistants,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompts = queries::prompts::my_prompts()
        .bind(&transaction, &db::PromptType::Assistant)
        .all()
        .await?;

    let html = my_assistants::index::page(team_id, rbac, prompts);

    Ok(Html(html))
}

pub async fn view_assistant(
    View { team_id, prompt_id }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let _datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let _integrations = queries::integrations::integrations()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;

    let categories = queries::categories::categories()
        .bind(&transaction)
        .all()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    // Parse selected dataset IDs from comma-separated string
    let _selected_dataset_ids: Vec<i32> = if prompt.selected_datasets.is_empty() {
        Vec::new()
    } else {
        prompt
            .selected_datasets
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    // Parse selected integration IDs from comma-separated string
    let _selected_integration_ids: Vec<i32> = if prompt.selected_integrations.is_empty() {
        Vec::new()
    } else {
        prompt
            .selected_integrations
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    let form = my_assistants::upsert::PromptForm {
        id: Some(prompt.id),
        name: prompt.name,
        system_prompt: prompt.system_prompt.unwrap_or_default(),
        models: models.clone(),
        categories: categories.clone(),
        visibility: visibility_to_string(prompt.visibility),
        model_id: prompt.model_id,
        category_id: prompt.category_id,
        max_history_items: prompt.max_history_items,
        max_chunks: prompt.max_chunks,
        max_tokens: prompt.max_tokens,
        trim_ratio: prompt.trim_ratio,
        temperature: prompt.temperature.unwrap_or(0.7),
        description: prompt.description,
        disclaimer: prompt.disclaimer,
        example1: prompt.example1,
        example2: prompt.example2,
        example3: prompt.example3,
        example4: prompt.example4,
        error: None,
    };

    let html = my_assistants::view::page(team_id, rbac, form);

    Ok(Html(html))
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

pub async fn manage_integrations(
    ManageIntegrations { team_id, prompt_id }: ManageIntegrations,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integrations = queries::integrations::integrations()
        .bind(&transaction)
        .all()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    // Parse selected integration IDs from comma-separated string
    let selected_integration_ids: Vec<i32> = if prompt.selected_integrations.is_empty() {
        Vec::new()
    } else {
        prompt
            .selected_integrations
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    let form = my_assistants::integrations::IntegrationForm {
        prompt_id: prompt.id,
        prompt_name: prompt.name,
        integrations,
        selected_integration_ids,
        error: None,
    };

    let html = my_assistants::integrations::page(team_id, rbac, form);

    Ok(Html(html))
}
