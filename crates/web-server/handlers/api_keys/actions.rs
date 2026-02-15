use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum_extra::extract::Form;
use db::authz;
use db::{queries, Pool};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde::Deserialize;
use validator::Validate;
use web_pages::api_keys::page::GeneratedKey;
use web_pages::routes::api_keys::{Delete, New};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewApiKey {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub prompt_id: i32,
}

pub async fn action_new_api_key(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(new_api_key): Form<NewApiKey>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let mut generated_key: Option<GeneratedKey> = None;

    if new_api_key.validate().is_ok() {
        let api_key_value: String = rng()
            .sample_iter(&Alphanumeric)
            .take(40)
            .map(char::from)
            .collect();

        queries::api_keys::new_api_key()
            .bind(
                &transaction,
                &new_api_key.prompt_id,
                &rbac.user_id,
                &team_id_num,
                &new_api_key.name,
                &api_key_value,
            )
            .await?;

        if let Ok(prompt) = queries::prompts::prompt()
            .bind(&transaction, &new_api_key.prompt_id, &team_id_num)
            .one()
            .await
        {
            generated_key = Some(GeneratedKey {
                name: new_api_key.name.clone(),
                value: api_key_value,
                prompt_name: Some(prompt.name),
                prompt_type: Some(prompt.prompt_type),
            });
        } else {
            generated_key = Some(GeneratedKey {
                name: new_api_key.name.clone(),
                value: api_key_value,
                prompt_name: None,
                prompt_type: None,
            });
        }
    }

    let page_data = super::page_data::load_api_keys_page_data(&transaction, team_id_num).await?;

    transaction.commit().await?;

    let html = web_pages::api_keys::page::page(
        rbac,
        team_id,
        page_data.api_keys,
        page_data.assistants,
        page_data.models,
        page_data.token_usage_data,
        page_data.api_request_data,
        generated_key,
    );

    Ok(Html(html))
}

pub async fn action_delete_api_key(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (_permissions, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    queries::api_keys::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::api_keys::Index { team_id }.to_string(),
        "Document Deleted",
    )
}
