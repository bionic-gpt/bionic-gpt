use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries::api_keys;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::api_keys::{Index, New};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewApiKey {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub prompt_id: i32,
}

pub async fn new_api_key(
    New { team_id }: New,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(new_api_key): Form<NewApiKey>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if new_api_key.validate().is_ok() {
        let api_key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        api_keys::new_api_key()
            .bind(
                &transaction,
                &new_api_key.prompt_id,
                &rbac.user_id,
                &team_id,
                &new_api_key.name,
                &api_key,
            )
            .await?;
    }

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(&Index { team_id }.to_string(), "Api Key Created")
}
