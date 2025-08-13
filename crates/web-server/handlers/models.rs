use crate::layout::empty_string_is_none;
use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::authz;
use db::queries;
use db::queries::models;
use db::ModelCapability;
use db::ModelType;
use db::Pool;
use db::Visibility;
// Add capabilities module
use db::queries::capabilities;
use serde::Deserialize;
use validator::Validate;
use web_pages::models::upsert as model_page;
use web_pages::routes::models::{Delete, Edit, Index, New, Upsert};
use web_pages::{string_to_visibility, visibility_to_string};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_get(new_loader)
        .typed_get(edit_loader)
        .typed_post(upsert_action)
        .typed_post(delete_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_setup_models() {
        return Err(CustomError::Authorization);
    }

    let models = models::all_models().bind(&transaction).all().await?;

    // For each model, fetch its capabilities
    let mut models_with_capabilities = Vec::new();
    for model in models {
        let capabilities = capabilities::get_model_capabilities()
            .bind(&transaction, &model.id)
            .all()
            .await?;

        let has_function_calling = capabilities
            .iter()
            .any(|c| c.capability == ModelCapability::function_calling);
        let has_vision = capabilities
            .iter()
            .any(|c| c.capability == ModelCapability::vision);
        let has_tool_use = capabilities
            .iter()
            .any(|c| c.capability == ModelCapability::tool_use);
        let has_guard = capabilities
            .iter()
            .any(|c| c.capability == ModelCapability::Guarded);

        models_with_capabilities.push((
            model,
            has_function_calling,
            has_vision,
            has_tool_use,
            has_guard,
        ));
    }

    let html = web_pages::models::page::page(team_id, rbac, models_with_capabilities);

    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_setup_models() {
        return Err(CustomError::Authorization);
    }

    let form = model_page::ModelForm {
        id: None,
        prompt_id: None,
        name: "".to_string(),
        display_name: "".to_string(),
        model_type: "LLM".to_string(),
        base_url: "".to_string(),
        api_key: "".to_string(),
        tpm_limit: 10_000,
        rpm_limit: 10_000,
        context_size_bytes: 2048,
        visibility: visibility_to_string(if rbac.is_sys_admin {
            Visibility::Company
        } else {
            Visibility::Team
        }),
        description: "".to_string(),
        disclaimer: "AI can make mistakes. Check important information.".to_string(),
        example1: "".to_string(),
        example2: "".to_string(),
        example3: "".to_string(),
        example4: "".to_string(),
        has_capability_function_calling: false,
        has_capability_vision: false,
        has_capability_tool_use: false,
        has_capability_guard: false,
        error: None,
    };

    let html = model_page::page(team_id, rbac, form);

    Ok(Html(html))
}

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_setup_models() {
        return Err(CustomError::Authorization);
    }

    let model = models::model_with_prompt()
        .bind(&transaction, &id)
        .one()
        .await?;

    let visibility = if let Some(prompt_id) = model.prompt_id {
        let prompt = queries::prompts::prompt()
            .bind(&transaction, &prompt_id, &team_id)
            .one()
            .await?;
        visibility_to_string(prompt.visibility)
    } else {
        visibility_to_string(Visibility::Team)
    };

    let capabilities = capabilities::get_model_capabilities()
        .bind(&transaction, &id)
        .all()
        .await?;

    let has_function_calling = capabilities
        .iter()
        .any(|c| c.capability == ModelCapability::function_calling);
    let has_vision = capabilities
        .iter()
        .any(|c| c.capability == ModelCapability::vision);
    let has_tool_use = capabilities
        .iter()
        .any(|c| c.capability == ModelCapability::tool_use);
    let has_guard = capabilities
        .iter()
        .any(|c| c.capability == ModelCapability::Guarded);

    let model_type = match model.model_type {
        ModelType::LLM => "LLM".to_string(),
        ModelType::Image => "Image".to_string(),
        ModelType::Embeddings => "Embeddings".to_string(),
        ModelType::TextToSpeech => "TextToSpeech".to_string(),
        ModelType::Guard => "Guard".to_string(),
    };

    let form = model_page::ModelForm {
        id: Some(model.id),
        prompt_id: model.prompt_id,
        name: model.name,
        // Preserve existing form values when editing
        display_name: model.display_name.clone(),
        model_type,
        base_url: model.base_url,
        api_key: model.api_key.unwrap_or_default(),
        tpm_limit: model.tpm_limit,
        rpm_limit: model.rpm_limit,
        context_size_bytes: model.context_size,
        visibility,
        description: model.description.clone(),
        disclaimer: model.disclaimer,
        example1: model.example1,
        example2: model.example2,
        example3: model.example3,
        example4: model.example4,
        has_capability_function_calling: has_function_calling,
        has_capability_vision: has_vision,
        has_capability_tool_use: has_tool_use,
        has_capability_guard: has_guard,
        error: None,
    };

    let html = model_page::page(team_id, rbac, form);

    Ok(Html(html))
}

pub async fn delete_action(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_setup_models() {
        return Err(CustomError::Authorization);
    }

    queries::models::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::models::Index { team_id }.to_string(),
        "Model Deleted",
    )
}
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
    pub visibility: String,
    pub disclaimer: String,
    pub description: String,
    pub example1: String,
    pub example2: String,
    pub example3: String,
    pub example4: String,
    // Add capability fields
    pub capability_function_calling: Option<String>,
    pub capability_vision: Option<String>,
    pub capability_tool_use: Option<String>,
    pub capability_guard: Option<String>,
}

pub async fn upsert_action(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(model_form): Form<ModelForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_setup_models() {
        return Err(CustomError::Authorization);
    }

    let model_type = match model_form.model_type.as_str() {
        "LLM" => ModelType::LLM,
        "Image" => ModelType::Image,
        "TextToSpeech" => ModelType::TextToSpeech,
        "Guard" => ModelType::Guard,
        _ => ModelType::Embeddings,
    };

    let mut visibility = string_to_visibility(&model_form.visibility);
    if visibility == Visibility::Company && !rbac.is_sys_admin {
        visibility = Visibility::Team;
    }

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
                        &visibility,
                        &system_prompt,
                        &99,
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

            // Handle capabilities if it's an LLM model
            if model_type == ModelType::LLM {
                // First, delete all existing capabilities for this model
                capabilities::delete_all_model_capabilities()
                    .bind(&transaction, &model_id)
                    .await?;

                // Then add the selected capabilities
                if model_form.capability_function_calling.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::function_calling)
                        .await?;
                }

                if model_form.capability_vision.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::vision)
                        .await?;
                }

                if model_form.capability_tool_use.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::tool_use)
                        .await?;
                }
                if model_form.capability_guard.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::Guarded)
                        .await?;
                }
            }

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
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
                        &visibility,
                        &system_prompt,
                        &99,
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

            // Handle capabilities if it's an LLM model
            if model_type == ModelType::LLM {
                if model_form.capability_function_calling.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::function_calling)
                        .await?;
                }

                if model_form.capability_vision.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::vision)
                        .await?;
                }

                if model_form.capability_tool_use.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::tool_use)
                        .await?;
                }
                if model_form.capability_guard.is_some() {
                    capabilities::set_model_capability()
                        .bind(&transaction, &model_id, &ModelCapability::Guarded)
                        .await?;
                }
            }

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::models::Index { team_id }.to_string(),
                "Model Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::models::Index { team_id }.to_string(),
            "Problem with Model Validation",
        )
        .into_response()),
    }
}
