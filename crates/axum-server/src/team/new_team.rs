use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::rls;
use db::types;
use db::Pool;
use db::{queries, DatasetConnection, Visibility};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewTeam {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new_team(
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
    Form(new_team): Form<NewTeam>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    rls::set_row_level_security_user_id(&transaction, current_user.user_id).await?;

    let org_id = queries::teams::insert_team()
        .bind(&transaction)
        .one()
        .await?;

    let roles = vec![
        types::public::Role::SystemAdministrator,
        types::public::Role::Collaborator,
    ];

    queries::teams::add_user_to_team()
        .bind(&transaction, &current_user.user_id, &org_id, &roles)
        .await?;

    queries::teams::set_name()
        .bind(&transaction, &new_team.name, &org_id)
        .await?;

    let model = queries::models::get_system_model()
        .bind(&transaction)
        .one()
        .await?;

    let system_prompt: Option<String> = None;

    queries::prompts::insert()
        .bind(
            &transaction,
            &org_id,
            &model.id,
            &"Default (Exclude All Datasets)",
            &Visibility::Private,
            &DatasetConnection::None,
            &system_prompt,
            &3,
            &10,
            &1024,
            &80,
            &0.7,
            &0.1,
        )
        .one()
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::team::switch_route(org_id),
        "New Team Created",
    )
}
