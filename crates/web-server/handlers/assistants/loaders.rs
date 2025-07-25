use crate::{config::Config, CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::Pool;
use db::{queries, PromptType};
use web_pages::visibility_to_string;
use web_pages::{
    assistants,
    routes::prompts::{Edit, Index, New},
};

pub async fn index_loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Assistant)
        .all()
        .await?;

    let categories = queries::categories::categories()
        .bind(&transaction)
        .all()
        .await?;

    let html = assistants::page::page(team_id, rbac, prompts, categories);

    Ok(Html(html))
}

pub async fn new_assistant_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let _datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let _integrations = queries::integrations::integrations()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let models = queries::prompts::prompts()
        .bind(&transaction, &team_id, &PromptType::Model)
        .all()
        .await?;

    let categories = queries::categories::categories()
        .bind(&transaction)
        .all()
        .await?;

    let form = web_pages::my_assistants::upsert::PromptForm {
        id: None,
        name: "".to_string(),
        system_prompt: "".to_string(),
        models: models.clone(),
        categories: categories.clone(),
        visibility: "Private".to_string(),
        model_id: -1,
        category_id: -1,
        max_history_items: 99,
        max_chunks: 10,
        max_tokens: 1024,
        trim_ratio: 80,
        temperature: 0.7,
        description: "".to_string(),
        disclaimer: "LLMs can make mistakes. Check important info.".to_string(),
        example1: None,
        example2: None,
        example3: None,
        example4: None,
        error: None,
    };

    let show_company_visibility = rbac.can_make_assistant_public() && !config.saas;

    let html = web_pages::my_assistants::upsert::page(team_id, rbac, form, show_company_visibility);

    Ok(Html(html))
}

pub async fn edit_assistant_loader(
    Edit { team_id, prompt_id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let _datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let _integrations = queries::integrations::integrations()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let models = queries::prompts::prompts()
        .bind(&transaction, &team_id, &PromptType::Model)
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

    let form = web_pages::my_assistants::upsert::PromptForm {
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

    let show_company_visibility = rbac.can_make_assistant_public() && !config.saas;

    let html = web_pages::my_assistants::upsert::page(team_id, rbac, form, show_company_visibility);

    Ok(Html(html))
}
