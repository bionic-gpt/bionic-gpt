use crate::{CustomError, Jwt};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use db::authz;
use db::queries;
use db::Pool;
use sha2::{Digest, Sha256};
use web_pages::routes::team::AcceptInvite;

pub async fn invite(
    AcceptInvite {
        invite_selector,
        invite_validator,
    }: AcceptInvite,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<impl IntoResponse, CustomError> {
    let team_slug =
        accept_invitation(&pool, current_user, &invite_selector, &invite_validator).await?;

    Ok(Redirect::to(
        &web_pages::routes::teams::Switch { team_id: team_slug }.to_string(),
    ))
}

pub async fn accept_invitation(
    pool: &Pool,
    current_user: Jwt,
    invitation_selector: &str,
    invitation_verifier: &str,
) -> Result<String, CustomError> {
    let invitation_verifier = URL_SAFE_NO_PAD
        .decode(invitation_verifier)
        .map_err(|_| CustomError::Authorization)?;
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 = URL_SAFE_NO_PAD.encode(invitation_verifier_hash);

    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let user_id = authz::set_row_level_security_user_id(&transaction, current_user.sub).await?;

    let invitation = queries::invitations::consume_invitation()
        .bind(&transaction, &invitation_selector)
        .opt()
        .await?
        .ok_or(CustomError::Authorization)?;

    // The recipient-scoped deletion is rolled back on any validation or write failure.
    if invitation.invitation_verifier_hash != invitation_verifier_hash_base64 {
        return Err(CustomError::Authorization);
    }

    let user = queries::users::user()
        .bind(&transaction, &user_id)
        .one()
        .await?;

    queries::teams::add_user_to_team()
        .bind(
            &transaction,
            &user_id,
            &invitation.team_id,
            &invitation.roles,
        )
        .await?;

    if (None, None) == (user.first_name, user.last_name) {
        queries::users::set_name()
            .bind(
                &transaction,
                &invitation.first_name,
                &invitation.last_name,
                &user_id,
            )
            .await?;
    }

    let team_id = db::team_public_id::encode(invitation.team_id)
        .ok_or_else(|| CustomError::FaultySetup("Could not encode team id".to_string()))?;

    transaction.commit().await?;

    Ok(team_id)
}

#[cfg(test)]
#[path = "invitation_tests.rs"]
mod invitation_tests;
