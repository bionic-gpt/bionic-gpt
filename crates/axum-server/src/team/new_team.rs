use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries;
use db::types;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewTeam {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new_team(
    Extension(pool): Extension<Pool>,
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(new_team): Form<NewTeam>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let org_id = queries::organisations::insert_organisation()
        .bind(&transaction)
        .one()
        .await?;

    let roles = vec![
        types::public::Role::Administrator,
        types::public::Role::Collaborator,
    ];

    queries::organisations::add_user_to_organisation()
        .bind(&transaction, &current_user.user_id, &org_id, &roles)
        .await?;

    queries::organisations::set_name()
        .bind(&transaction, &new_team.name, &org_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::team::switch_route(organisation_id),
        "New Team Created",
    )
}
