use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::types;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::teams::New;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewTeam {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new_team(
    New { team_id: _ }: New,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
    Form(new_team): Form<NewTeam>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let user_id = authz::set_row_level_security_user_id(&transaction, current_user.sub).await?;

    let team_id = queries::teams::insert_team()
        .bind(&transaction)
        .one()
        .await?;

    let roles = vec![types::public::Role::Collaborator];

    queries::teams::add_user_to_team()
        .bind(&transaction, &user_id, &team_id, &roles)
        .await?;

    queries::teams::set_name()
        .bind(&transaction, &new_team.name, &team_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::teams::Switch { team_id }.to_string(),
        "New Team Created",
    )
}
