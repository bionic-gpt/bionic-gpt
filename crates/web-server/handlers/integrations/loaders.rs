use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use db::{authz, queries, Pool};
use include_dir::{include_dir, Dir};
use integrations::bionic_openapi::BionicOpenAPI;
use serde_json::Value;
use web_pages::integrations::integration_card::IntegrationSummary;
use web_pages::integrations::select::PrebuiltSpec;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::routes::integrations::View;
use web_pages::routes::integrations::{Edit, Index, New, Select};

static PREBUILT_SPECS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../mcp/specs");

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integrations_db = queries::integrations::integrations()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    // Build integration summaries with connection counts
    let mut integrations: Vec<IntegrationSummary> = Vec::new();
    for integration in integrations_db.iter() {
        if let Some(definition) = &integration.definition {
            if let Ok(bionic_openapi) = BionicOpenAPI::new(definition) {
                let api_key_count = if bionic_openapi.has_api_key_security() {
                    queries::connections::get_api_key_connections_for_integration()
                        .bind(&transaction, &integration.id, &team_id)
                        .all()
                        .await?
                        .len()
                } else {
                    0
                };

                let (oauth2_count, oauth_client_configured) =
                    if bionic_openapi.has_oauth2_security() {
                        let count = queries::connections::get_oauth2_connections_for_integration()
                            .bind(&transaction, &integration.id, &team_id)
                            .all()
                            .await?
                            .len();

                        let has_client = if let Some(config) = bionic_openapi.get_oauth2_config() {
                            !queries::oauth_clients::oauth_client_by_provider_url()
                                .bind(&transaction, &config.authorization_url)
                                .all()
                                .await?
                                .is_empty()
                        } else {
                            false
                        };

                        (count, has_client)
                    } else {
                        (0, false)
                    };

                integrations.push(IntegrationSummary {
                    openapi: bionic_openapi,
                    id: integration.id,
                    api_key_count,
                    oauth2_count,
                    oauth_client_configured,
                });
            }
        } else {
            tracing::error!("This integration doesn't have a definition");
        }
    }

    let html = web_pages::integrations::page::page(team_id, rbac, integrations);

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
        .bind(&transaction, &id, &team_id)
        .one()
        .await?;

    let (
        tool_definitions,
        openapi,
        api_key_connections,
        oauth2_connections,
        oauth_client_configured,
    ) = if let Some(definition) = &integration.definition {
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

                let (oauth2_connections, oauth_client_configured) =
                    if openapi_helper.has_oauth2_security() {
                        let connections =
                            queries::connections::get_oauth2_connections_for_integration()
                                .bind(&transaction, &id, &team_id)
                                .all()
                                .await
                                .unwrap_or_default();

                        let has_client = if let Some(config) = openapi_helper.get_oauth2_config() {
                            !queries::oauth_clients::oauth_client_by_provider_url()
                                .bind(&transaction, &config.authorization_url)
                                .all()
                                .await?
                                .is_empty()
                        } else {
                            false
                        };

                        (connections, has_client)
                    } else {
                        (vec![], false)
                    };

                (
                    integration_tools.tool_definitions,
                    openapi_helper,
                    api_key_connections,
                    oauth2_connections,
                    oauth_client_configured,
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
                    false,
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
            false,
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
        oauth_client_configured,
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

    let integration_form = IntegrationForm {
        visibility: web_pages::visibility_to_string(db::Visibility::Private),
        ..Default::default()
    };

    let html = web_pages::integrations::upsert::page(team_id, rbac, integration_form);

    Ok(Html(html))
}

pub async fn select_loader(
    Select { team_id }: Select,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let mut specs: Vec<PrebuiltSpec> = PREBUILT_SPECS
        .files()
        .filter_map(|file| file.contents_utf8().map(|contents| (file, contents)))
        .filter_map(|(file, contents)| {
            serde_json::from_str::<Value>(contents)
                .map_err(|error| {
                    tracing::warn!("Failed to parse spec {}: {}", file.path().display(), error);
                    error
                })
                .ok()
                .and_then(|value| {
                    let title = value
                        .get("info")
                        .and_then(|info| info.get("title"))
                        .and_then(|title| title.as_str())
                        .map(|title| title.to_string())
                        .or_else(|| {
                            file.path()
                                .file_stem()
                                .map(|stem| stem.to_string_lossy().to_string())
                        });

                    title.map(|title| {
                        let description = value
                            .get("info")
                            .and_then(|info| info.get("description"))
                            .and_then(|desc| desc.as_str())
                            .map(|desc| desc.to_string());

                        let logo_data_url = value
                            .get("info")
                            .and_then(|info| info.get("x-logo"))
                            .and_then(|logo| {
                                logo.get("url")
                                    .and_then(|url| url.as_str())
                                    .map(|url| url.to_string())
                                    .or_else(|| logo.as_str().map(|url| url.to_string()))
                            });

                        let spec_json =
                            serde_json::to_string(&value).unwrap_or_else(|_| contents.to_string());

                        PrebuiltSpec {
                            file_name: file
                                .path()
                                .file_stem()
                                .map(|stem| stem.to_string_lossy().to_string())
                                .unwrap_or_else(|| "integration".to_string()),
                            title,
                            description,
                            spec_json,
                            logo_data_url,
                        }
                    })
                })
        })
        .collect();

    specs.sort_by(|a, b| a.title.cmp(&b.title));

    let html = web_pages::integrations::select::page(team_id, rbac, specs);

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
        .bind(&transaction, &id, &team_id)
        .one()
        .await?;

    let integration_form = if let Some(definition) = &integration.definition {
        IntegrationForm {
            id: Some(integration.id),
            openapi_spec: serde_json::to_string(&definition).unwrap_or("".to_string()),
            visibility: web_pages::visibility_to_string(integration.visibility),
            error: None,
        }
    } else {
        IntegrationForm {
            visibility: web_pages::visibility_to_string(integration.visibility),
            ..Default::default()
        }
    };

    let html = web_pages::integrations::upsert::page(team_id, rbac, integration_form);

    Ok(Html(html))
}
