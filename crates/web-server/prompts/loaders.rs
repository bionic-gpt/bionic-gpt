use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::Pool;
use db::{queries, ModelType};
use web_pages::visibility_to_string;
use web_pages::{
    prompts,
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

    let html = prompts::index::page(team_id, rbac, prompts, categories);

    Ok(Html(html))
}

pub async fn new_assistant_loader(
    New { team_id }: New,
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

    let models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;

    let categories = queries::categories::categories()
        .bind(&transaction)
        .all()
        .await?;

    let form = prompts::upsert::PromptForm {
        id: None,
        name: "".to_string(),
        system_prompt: "".to_string(),
        datasets: datasets.clone(),
        selected_dataset_ids: Default::default(),
        models: models.clone(),
        categories: categories.clone(),
        visibility: "Private".to_string(),
        model_id: -1,
        category_id: -1,
        max_history_items: 3,
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

    let html = prompts::upsert::page(team_id, rbac, form);

    Ok(Html(html))
}

pub async fn edit_assistant_loader(
    Edit { team_id, prompt_id }: Edit,
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

    let form = prompts::upsert::PromptForm {
        id: None,
        name: prompt.name,
        system_prompt: prompt.system_prompt.unwrap_or_default(),
        datasets: datasets.clone(),
        selected_dataset_ids: Default::default(),
        models: models.clone(),
        categories: categories.clone(),
        visibility: visibility_to_string(prompt.visibility),
        model_id: -1,
        category_id: -1,
        max_history_items: 3,
        max_chunks: 10,
        max_tokens: 1024,
        trim_ratio: 80,
        temperature: 0.7,
        description: prompt.description,
        disclaimer: "LLMs can make mistakes. Check important info.".to_string(),
        example1: None,
        example2: None,
        example3: None,
        example4: None,
        error: None,
    };

    let html = prompts::upsert::page(team_id, rbac, form);

    Ok(Html(html))
}
