use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SetName {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn set_name(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(set_name): Form<SetName>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::teams::set_name()
        .bind(&transaction, &set_name.name, &team_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::team::index_route(team_id),
        "Team Name Updated",
    )
}
