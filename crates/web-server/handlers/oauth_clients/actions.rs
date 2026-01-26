use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum_extra::extract::Form;
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::oauth_clients::{Delete, New};

pub async fn action_delete(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    queries::oauth_clients::delete_oauth_client()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::oauth_clients::Index { team_id }.to_string(),
        "OAuth Client Deleted",
    )
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct OauthClientForm {
    #[validate(length(min = 1, message = "Client ID is required"))]
    pub client_id: String,
    #[validate(length(min = 1, message = "Client Secret is required"))]
    pub client_secret: String,
    #[validate(length(min = 1, message = "Provider is required"))]
    pub provider: String,
    #[validate(length(min = 1, message = "Provider URL is required"))]
    pub provider_url: String,
}

pub async fn action_create(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(oauth_client_form): Form<OauthClientForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    match oauth_client_form.validate() {
        Ok(_) => {
            // Check if an OAuth client already exists for this provider URL
            let existing = queries::oauth_clients::oauth_client_by_provider_url()
                .bind(&transaction, &oauth_client_form.provider_url)
                .all()
                .await?;

            if !existing.is_empty() {
                let oauth_client = web_pages::oauth_clients::upsert::OauthClientForm {
                    client_id: oauth_client_form.client_id,
                    client_secret: oauth_client_form.client_secret,
                    provider: oauth_client_form.provider,
                    provider_url: oauth_client_form.provider_url,
                    error: Some(
                        "An OAuth client with this provider URL already exists".to_string(),
                    ),
                };
                let html = web_pages::oauth_clients::upsert::page(team_id, rbac, oauth_client);
                return Ok(Html(html).into_response());
            }

            queries::oauth_clients::insert_oauth_client()
                .bind(
                    &transaction,
                    &oauth_client_form.client_id,
                    &oauth_client_form.client_secret,
                    &oauth_client_form.provider,
                    &oauth_client_form.provider_url,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::oauth_clients::Index { team_id }.to_string(),
                "OAuth Client Created",
            )
            .into_response())
        }
        Err(_) => {
            let oauth_client = web_pages::oauth_clients::upsert::OauthClientForm {
                client_id: oauth_client_form.client_id,
                client_secret: oauth_client_form.client_secret,
                provider: oauth_client_form.provider,
                provider_url: oauth_client_form.provider_url,
                error: Some("Please check the form for errors".to_string()),
            };
            let html = web_pages::oauth_clients::upsert::page(team_id, rbac, oauth_client);
            Ok(Html(html).into_response())
        }
    }
}
