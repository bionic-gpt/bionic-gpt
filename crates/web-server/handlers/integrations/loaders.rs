use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use db::{authz, queries, Pool};
use integrations::bionic_openapi::BionicOpenAPI;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::routes::integrations::View;
use web_pages::routes::integrations::{Edit, Index, New};

pub async fn loader(
    Index { team_id }: Index,
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

    // Get the Open API Spec
    let integrations: Vec<(BionicOpenAPI, i32)> = integrations
        .iter()
        .filter_map(|integration| {
            if let Some(definition) = &integration.definition {
                if let Ok(bionic_openapi) = BionicOpenAPI::new(definition) {
                    Some((bionic_openapi, integration.id))
                } else {
                    None
                }
            } else {
                tracing::error!("This integration doesn't have a definition");
                None
            }
        })
        .collect();

    let html = web_pages::integrations::index::page(team_id, rbac, integrations);

    Ok(Html(html))
}

pub async fn view_loader(
    View { team_id, id }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration = db::queries::integrations::integration()
        .bind(&transaction, &id)
        .one()
        .await?;

    let (tool_definitions, openapi, api_key_connections, oauth2_connections) =
        if let Some(definition) = &integration.definition {
            match BionicOpenAPI::new(definition) {
                Ok(openapi_helper) => {
                    let integration_tools = openapi_helper.create_tool_definitions();

                    // Fetch connections based on security type
                    let api_key_connections = if openapi_helper.has_api_key_security() {
                        queries::connections::get_api_key_connections_for_integration()
                            .bind(&transaction, &id, &team_id)
                            .all()
                            .await
                            .unwrap_or_default()
                    } else {
                        vec![]
                    };

                    let oauth2_connections = if openapi_helper.has_oauth2_security() {
                        queries::connections::get_oauth2_connections_for_integration()
                            .bind(&transaction, &id, &team_id)
                            .all()
                            .await
                            .unwrap_or_default()
                    } else {
                        vec![]
                    };

                    (
                        integration_tools.tool_definitions,
                        openapi_helper,
                        api_key_connections,
                        oauth2_connections,
                    )
                }
                Err(_) => {
                    // If parsing fails, use defaults
                    (
                        vec![],
                        BionicOpenAPI::new(&serde_json::json!({}))
                            .unwrap_or_else(|_| panic!("Failed to create default BionicOpenAPI")),
                        vec![],
                        vec![],
                    )
                }
            }
        } else {
            (
                vec![],
                BionicOpenAPI::new(&serde_json::json!({}))
                    .unwrap_or_else(|_| panic!("Failed to create default BionicOpenAPI")),
                vec![],
                vec![],
            )
        };

    let html = web_pages::integrations::view::view(
        team_id,
        rbac,
        integration,
        tool_definitions,
        openapi,
        api_key_connections,
        oauth2_connections,
    );

    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration_form = IntegrationForm::default();

    let html = web_pages::integrations::upsert::page(team_id, rbac, integration_form);

    Ok(Html(html))
}

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration = queries::integrations::integration()
        .bind(&transaction, &id)
        .one()
        .await?;

    let integration_form = if let Some(definition) = &integration.definition {
        IntegrationForm {
            id: Some(integration.id),
            openapi_spec: serde_json::to_string(&definition).unwrap_or("".to_string()),
            error: None,
        }
    } else {
        IntegrationForm::default()
    };

    let html = web_pages::integrations::upsert::page(team_id, rbac, integration_form);

    Ok(Html(html))
}
