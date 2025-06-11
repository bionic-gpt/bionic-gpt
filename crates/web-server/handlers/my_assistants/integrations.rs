use crate::{CustomError, Jwt};
use axum::response::Html;
use axum::{extract::Extension, response::IntoResponse};
use axum_extra::extract::Form;
use db::{authz, queries, Pool, Transaction};
use serde::Deserialize;
use std::collections::HashMap;
use validator::Validate;
use web_pages::{
    my_assistants,
    routes::prompts::{AddIntegration, ManageIntegrations, RemoveIntegration, UpdateIntegrations},
};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationUpdateForm {
    #[serde(default)]
    pub integrations: Vec<i32>,
    #[serde(default)]
    pub integration_connections: HashMap<String, my_assistants::integrations::ConnectionSelection>,
}

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

async fn update_integrations_with_connections(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    integrations: Vec<i32>,
    integration_connections: HashMap<String, my_assistants::integrations::ConnectionSelection>,
) -> Result<(), CustomError> {
    for integration_id in integrations {
        let connection_key = integration_id.to_string();
        let connection = integration_connections.get(&connection_key);

        queries::prompt_integrations::insert_prompt_integration_with_connection()
            .bind(
                transaction,
                &prompt_id,
                &integration_id,
                &connection.and_then(|c| c.api_connection_id),
                &connection.and_then(|c| c.oauth2_connection_id),
            )
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

    // Add new integration connections with connection info
    update_integrations_with_connections(
        &transaction,
        prompt_id,
        form.integrations,
        form.integration_connections,
    )
    .await?;

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

    // Analyze each integration for auth requirements
    let mut integrations_with_auth: Vec<my_assistants::integrations::IntegrationWithAuthInfo> =
        Vec::new();
    let mut available_connections: HashMap<i32, my_assistants::integrations::AvailableConnections> =
        HashMap::new();

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

            available_connections.insert(
                integration.id,
                my_assistants::integrations::AvailableConnections {
                    api_key_connections: api_connections,
                    oauth2_connections,
                },
            );

            integrations_with_auth.push(my_assistants::integrations::IntegrationWithAuthInfo {
                integration,
                requires_api_key,
                requires_oauth2,
                has_connections,
            });
        } else {
            integrations_with_auth.push(my_assistants::integrations::IntegrationWithAuthInfo {
                integration,
                requires_api_key: false,
                requires_oauth2: false,
                has_connections: true, // No auth required, so always "available"
            });
        }
    }

    tracing::debug!("get_prompt_integrations_with_connections");

    // Get existing prompt integrations with connections
    let existing_connections =
        queries::prompt_integrations::get_prompt_integrations_with_connections()
            .bind(&transaction, &prompt_id)
            .all()
            .await?;

    tracing::debug!("Finished get_prompt_integrations_with_connections");

    let mut integration_connections: HashMap<
        i32,
        my_assistants::integrations::ConnectionSelection,
    > = HashMap::new();
    let mut selected_integration_ids: Vec<i32> = Vec::new();

    for existing in existing_connections {
        selected_integration_ids.push(existing.integration_id);
        integration_connections.insert(
            existing.integration_id,
            my_assistants::integrations::ConnectionSelection {
                api_connection_id: existing.api_connection_id,
                oauth2_connection_id: existing.oauth2_connection_id,
            },
        );
    }

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let form = my_assistants::integrations::IntegrationForm {
        prompt_id: prompt.id,
        prompt_name: prompt.name,
        integrations: integrations_with_auth,
        selected_integration_ids,
        integration_connections,
        available_connections,
        error: None,
    };

    let html = my_assistants::integrations::page(team_id, rbac, form);

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
