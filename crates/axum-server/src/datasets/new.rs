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
pub struct NewDataset {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new(
    Extension(pool): Extension<Pool>,
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(new_dataset): Form<NewDataset>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::datasets::insert()
        .bind(&transaction, &organisation_id, &new_dataset.name)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_components::routes::datasets::index_route(organisation_id),
        "Dataset Created",
    )
}
