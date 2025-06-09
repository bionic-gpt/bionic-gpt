use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Form;
use db::{authz, queries, Pool, Visibility};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::integrations::{ConfigureApiKey, DeleteApiKeyConnection};

#[derive(Deserialize, Validate, Debug)]
pub struct ApiKeyForm {
    #[validate(length(min = 1, message = "API key is required"))]
    pub api_key: String,
    pub visibility: String,
}

pub async fn configure_api_key_action(
    ConfigureApiKey {
        team_id,
        integration_id,
    }: ConfigureApiKey,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(api_key_form): Form<ApiKeyForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Parse visibility
    let visibility = match api_key_form.visibility.as_str() {
        "Private" => Visibility::Private,
        "Team" => Visibility::Team,
        _ => Visibility::Private,
    };

    // Validate the form
    match api_key_form.validate() {
        Ok(_) => {
            // Insert new connection
            let _connection_id = queries::connections::insert_api_key_connection()
                .bind(
                    &transaction,
                    &integration_id,
                    &team_id,
                    &visibility,
                    &api_key_form.api_key,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::integrations::Index { team_id }.to_string(),
                "API Key configured successfully",
            ))
        }
        Err(_) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::integrations::Index { team_id }.to_string(),
            "Invalid API key configuration",
        )),
    }
}

pub async fn delete_api_key_connection_action(
    DeleteApiKeyConnection {
        team_id,
        integration_id,
        connection_id,
    }: DeleteApiKeyConnection,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Delete the connection
    queries::connections::delete_api_key_connection()
        .bind(&transaction, &connection_id, &team_id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::integrations::View {
            team_id,
            id: integration_id,
        }
        .to_string(),
        "API Key connection deleted successfully",
    ))
}
