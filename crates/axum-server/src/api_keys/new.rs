use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::api_keys;
use db::Pool;
use serde::Deserialize;
use ui_pages::routes::api_keys::index_route;
use validator::Validate;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewApiKey {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub prompt_id: i32,
}

pub async fn new_api_key(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(new_api_key): Form<NewApiKey>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

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
                &current_user.user_id,
                &team_id,
                &new_api_key.name,
                &api_key,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&index_route(team_id), "Api Key Created")
}
