use crate::{CustomError, Jwt};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
    Form,
};
use db::{authz, queries, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::teams::AcceptInvite;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct AcceptInviteForm {
    pub invite_id: i32,
}

pub async fn accept_invite(
    AcceptInvite {}: AcceptInvite,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(accept_invite): Form<AcceptInviteForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let user_id = authz::set_row_level_security_user_id(&transaction, current_user.sub).await?;

    let invitation = queries::invitations::consume_invitation_by_id()
        .bind(&transaction, &accept_invite.invite_id)
        .opt()
        .await?
        .ok_or(CustomError::Authorization)?;

    queries::teams::add_user_to_team()
        .bind(
            &transaction,
            &user_id,
            &invitation.team_id,
            &invitation.roles,
        )
        .await?;

    let team_id = db::team_public_id::encode(invitation.team_id)
        .ok_or_else(|| CustomError::FaultySetup("Could not encode team id".to_string()))?;

    transaction.commit().await?;

    Ok(Redirect::to(
        &web_pages::routes::teams::Switch { team_id }.to_string(),
    ))
}
