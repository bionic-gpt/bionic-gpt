use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries;
use db::rls;
use db::types;
use db::Pool;
use lettre::Message;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewInvite {
    #[validate(length(min = 1, message = "The email is mandatory"))]
    pub email: String,
    #[validate(length(min = 1, message = "The first name is mandatory"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "The last name is mandatory"))]
    pub last_name: String,
    pub admin: Option<String>,
}

pub async fn create_invite(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    authentication: Authentication,
    Extension(config): Extension<crate::config::Config>,
    Form(new_invite): Form<NewInvite>,
) -> Result<impl IntoResponse, CustomError> {
    let invite_hash = create(&pool, authentication, &new_invite, team_id).await?;

    let invitation_verifier_base64 = invite_hash.0;
    let invitation_selector_base64 = invite_hash.1;

    if let Some(smtp_config) = &config.smtp_config {
        let url = format!(
            "{}/app/invite/{}/{}",
            smtp_config.domain, invitation_selector_base64, invitation_verifier_base64
        );

        let body = format!(
            "
                Click {} to accept the invite
            ",
            url
        )
        .trim()
        .to_string();

        let email = Message::builder()
            .from(smtp_config.from_email.clone())
            .to(new_invite.email.parse().unwrap())
            .subject("You are invited to a Team")
            .body(body)
            .unwrap();

        crate::email::send_email(&config, email)
    }

    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions =
        rls::set_row_level_security_user(&transaction, current_user.sub, team_id).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::team::index_route(team.id),
        "Invitation Created",
    )
}

pub async fn create(
    pool: &Pool,
    current_user: Authentication,
    new_invite: &NewInvite,
    team_id: i32,
) -> Result<(String, String), CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions =
        rls::set_row_level_security_user(&transaction, current_user.sub, team_id).await?;

    let invitation_selector = rand::thread_rng().gen::<[u8; 6]>();
    let invitation_selector_base64 =
        base64::encode_config(invitation_selector, base64::URL_SAFE_NO_PAD);
    let invitation_verifier = rand::thread_rng().gen::<[u8; 8]>();
    let invitation_verifier_hash = Sha256::digest(invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE_NO_PAD);
    let invitation_verifier_base64 =
        base64::encode_config(invitation_verifier, base64::URL_SAFE_NO_PAD);

    let roles = if new_invite.admin.is_some() {
        vec![
            types::public::Role::SystemAdministrator,
            types::public::Role::Collaborator,
        ]
    } else {
        vec![types::public::Role::Collaborator]
    };

    queries::invitations::insert_invitation()
        .bind(
            &transaction,
            &team_id,
            &new_invite.email,
            &new_invite.first_name,
            &new_invite.last_name,
            &invitation_selector_base64,
            &invitation_verifier_hash_base64,
            &roles,
        )
        .await?;

    transaction.commit().await?;

    Ok((invitation_verifier_base64, invitation_selector_base64))
}
