use crate::{CustomError, Jwt};
use axum::response::Html;
use axum::{extract::Extension, response::IntoResponse};
use axum_extra::extract::Form;
use db::{authz, queries, Pool, Transaction};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    my_assistants,
    routes::prompts::{ManageIntegrations, UpdateIntegrations},
};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationUpdateForm {
    #[serde(default)]
    pub integrations: Vec<i32>,
}

async fn update_integrations(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    integrations: Vec<i32>,
) -> Result<(), CustomError> {
    for integration in integrations {
        queries::prompt_integrations::insert_prompt_integration()
            .bind(transaction, &prompt_id, &integration)
            .await?;
    }
    Ok(())
}

pub async fn update_integrations_action(
    UpdateIntegrations { team_id, prompt_id }: UpdateIntegrations,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<IntegrationUpdateForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Delete existing integration connections
    queries::prompt_integrations::delete_prompt_integrations()
        .bind(&transaction, &prompt_id)
        .await?;

    // Add new integration connections
    update_integrations(&transaction, prompt_id, form.integrations).await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::prompts::View { team_id, prompt_id }.to_string(),
        "Integration connections updated successfully",
    )
    .into_response())
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
