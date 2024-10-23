use super::super::{Authentication, CustomError};
use crate::config::Config;
use axum::{extract::Extension, response::IntoResponse};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use db::{authz, queries, Pool, Transaction, Visibility};
use validator::Validate;
use web_pages::{routes::prompts::Upsert, string_to_visibility};

#[derive(TryFromMultipart, Validate, Default, Debug)]
pub struct NewPromptTemplate {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub system_prompt: String,
    pub model_id: i32,
    pub category_id: i32,
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
    pub image_icon: Option<axum::body::Bytes>,
}

pub async fn upsert(
    Upsert { team_id }: Upsert,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    TypedMultipart(new_prompt_template): TypedMultipart<NewPromptTemplate>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let visibility = adjust_visibility(
        string_to_visibility(&new_prompt_template.visibility),
        config.saas,
    );

    let system_prompt = non_empty_string(&new_prompt_template.system_prompt);
    let image_bytes = new_prompt_template
        .image_icon
        .clone()
        .map(|image| image.to_vec());

    if new_prompt_template.validate().is_ok() {
        if let Some(id) = new_prompt_template.id {
            update_prompt(
                &transaction,
                &new_prompt_template,
                visibility,
                system_prompt,
                id,
            )
            .await?;
            update_datasets(&transaction, id, new_prompt_template.datasets).await?;
        } else {
            let prompt_id = insert_prompt(
                &transaction,
                &new_prompt_template,
                image_bytes,
                visibility,
                system_prompt,
                team_id,
            )
            .await?;
            update_datasets(&transaction, prompt_id, new_prompt_template.datasets).await?;
        }

        transaction.commit().await?;

        Ok(super::super::layout::redirect_and_snackbar(
            &web_pages::routes::prompts::MyPrompts { team_id }.to_string(),
            "Assistant Created",
        )
        .into_response())
    } else {
        Ok(super::super::layout::redirect_and_snackbar(
            &web_pages::routes::prompts::MyPrompts { team_id }.to_string(),
            "Failed to Create Assitant",
        )
        .into_response())
    }
}

fn adjust_visibility(mut visibility: Visibility, saas: bool) -> Visibility {
    if visibility == Visibility::Company && saas {
        visibility = Visibility::Team;
    }
    visibility
}

fn non_empty_string(s: &String) -> Option<&String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

async fn update_prompt(
    transaction: &Transaction<'_>,
    new_prompt_template: &NewPromptTemplate,
    visibility: Visibility,
    system_prompt: Option<&String>,
    id: i32,
) -> Result<(), CustomError> {
    queries::prompts::update()
        .bind(
            transaction,
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
        .bind(transaction, &id)
        .await?;
    if let Some(image) = &new_prompt_template.image_icon {
        queries::prompts::update_image()
            .bind(transaction, &image.to_vec(), &id)
            .await?;
    }
    Ok(())
}

async fn insert_prompt(
    transaction: &Transaction<'_>,
    new_prompt_template: &NewPromptTemplate,
    image_icon: Option<Vec<u8>>,
    visibility: Visibility,
    system_prompt: Option<&String>,
    team_id: i32,
) -> Result<i32, CustomError> {
    let id = queries::prompts::insert()
        .bind(
            transaction,
            &team_id,
            &new_prompt_template.model_id,
            &new_prompt_template.category_id,
            &new_prompt_template.name,
            &image_icon,
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
    Ok(id)
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
