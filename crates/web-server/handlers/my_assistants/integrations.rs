use crate::{locale::Locale, CustomError, Jwt};
use axum::response::Html;
use axum::{extract::Extension, response::IntoResponse};
use axum_extra::extract::Form;
use db::{authz, queries, Pool};
use serde::Deserialize;
use web_pages::{
    my_assistants,
    routes::prompts::{AddIntegration, ManageIntegrations, RemoveIntegration},
};

fn analyze_integration_auth(integration: &db::Integration) -> Result<(bool, bool), CustomError> {
    if let Some(definition) = &integration.definition {
        let bionic_api = integrations::bionic_openapi::BionicOpenAPI::new(definition)
            .map_err(|e| CustomError::FaultySetup(format!("Invalid OpenAPI spec: {}", e)))?;

        let requires_api_key = bionic_api.has_api_key_security();
        let requires_oauth2 = bionic_api.has_oauth2_security();

        Ok((requires_api_key, requires_oauth2))
    } else {
        Ok((false, false))
    }
}

pub async fn manage_integrations(
    ManageIntegrations { team_id, prompt_id }: ManageIntegrations,
    locale: Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integrations = queries::integrations::integrations()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    // Analyze each integration for auth requirements
    let mut integrations_with_auth: Vec<web_pages::shared::integrations::IntegrationWithAuthInfo> =
        Vec::new();

    for integration in integrations {
        let (requires_api_key, requires_oauth2) = analyze_integration_auth(&integration)?;

        if requires_api_key || requires_oauth2 {
            let api_connections = if requires_api_key {
                queries::connections::get_team_api_key_connections()
                    .bind(&transaction, &team_id, &integration.id)
                    .all()
                    .await?
            } else {
                Vec::new()
            };

            let oauth2_connections = if requires_oauth2 {
                queries::connections::get_team_oauth2_connections()
                    .bind(&transaction, &team_id, &integration.id)
                    .all()
                    .await?
            } else {
                Vec::new()
            };

            let has_connections = !api_connections.is_empty() || !oauth2_connections.is_empty();

            integrations_with_auth.push(web_pages::shared::integrations::IntegrationWithAuthInfo {
                integration,
                requires_api_key,
                requires_oauth2,
                has_connections,
                api_key_connections: api_connections,
                oauth2_connections,
            });
        } else {
            integrations_with_auth.push(web_pages::shared::integrations::IntegrationWithAuthInfo {
                integration,
                requires_api_key: false,
                requires_oauth2: false,
                has_connections: true, // No auth required, so always "available"
                api_key_connections: Vec::new(),
                oauth2_connections: Vec::new(),
            });
        }
    }

    // Get existing prompt integrations with connections
    let existing_connections =
        queries::prompt_integrations::get_prompt_integrations_with_connections()
            .bind(&transaction, &prompt_id)
            .all()
            .await?;

    let mut selected_integration_ids: Vec<i32> = Vec::new();

    for existing in existing_connections {
        selected_integration_ids.push(existing.integration_id);
    }

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let form = web_pages::shared::integrations::IntegrationForm {
        prompt_id: prompt.id,
        prompt_name: prompt.name,
        integrations: integrations_with_auth,
        selected_integration_ids,
        error: None,
    };

    let i18n = db::i18n::global();
    i18n.ensure_locale("en").await;
    if locale.as_str() != "en" {
        i18n.ensure_locale(locale.as_str()).await;
    }

    let html = my_assistants::integrations::page(team_id, rbac, form, locale.as_str());

    Ok(Html(html))
}

#[derive(Deserialize, Default, Debug)]
pub struct AddIntegrationForm {
    #[serde(default)]
    pub api_connection_id: Option<i32>,
    #[serde(default)]
    pub oauth2_connection_id: Option<i32>,
}

pub async fn add_integration_action(
    AddIntegration {
        team_id,
        prompt_id,
        integration_id,
    }: AddIntegration,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<AddIntegrationForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Add the integration with connection info
    queries::prompt_integrations::insert_prompt_integration_with_connection()
        .bind(
            &transaction,
            &prompt_id,
            &integration_id,
            &form.api_connection_id,
            &form.oauth2_connection_id,
        )
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::prompts::ManageIntegrations { team_id, prompt_id }.to_string(),
        "Integration added successfully",
    )
    .into_response())
}

pub async fn remove_integration_action(
    RemoveIntegration {
        team_id,
        prompt_id,
        integration_id,
    }: RemoveIntegration,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Remove the specific integration
    queries::prompt_integrations::delete_specific_prompt_integration()
        .bind(&transaction, &prompt_id, &integration_id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::prompts::ManageIntegrations { team_id, prompt_id }.to_string(),
        "Integration removed successfully",
    )
    .into_response())
}
