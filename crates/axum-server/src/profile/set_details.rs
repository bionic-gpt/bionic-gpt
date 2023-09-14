use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SetDetails {
    #[validate(length(min = 1, message = "The first name is mandatory"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "The last name is mandatory"))]
    pub last_name: String,
}

pub async fn set_details(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(set_name): Form<SetDetails>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::users::set_name()
        .bind(
            &transaction,
            &set_name.first_name,
            &set_name.last_name,
            &current_user.user_id,
        )
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &crate::profile::index_route(organisation_id),
        "Details Updated",
    )
}
