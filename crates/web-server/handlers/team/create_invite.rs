use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use db::authz;
use db::queries;
use db::types;
use db::Pool;
use lettre::Message;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use validator::Validate;
use web_pages::routes::team::{AcceptInvite, CreateInvite};

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
    CreateInvite { team_id }: CreateInvite,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    authentication: Jwt,
    Extension(config): Extension<crate::config::Config>,
    Form(new_invite): Form<NewInvite>,
) -> Result<impl IntoResponse, CustomError> {
    let invite_hash = create(&pool, authentication, &new_invite, &team_id).await?;

    if let Some(smtp_config) = &config.smtp_config {
        let url = AcceptInvite {
            invite_selector: invite_hash.1,
            invite_validator: invite_hash.0,
        }
        .to_string();

        let url = format!("{}{}", smtp_config.domain, url);

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
    let (_permissions, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    let team_public_id = db::team_public_id::encode(team_id_num)
        .ok_or_else(|| CustomError::FaultySetup("Could not encode team id".to_string()))?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::team::Index {
            team_id: team_public_id,
        }
        .to_string(),
        "Invitation Created",
    )
}

pub async fn create(
    pool: &Pool,
    current_user: Jwt,
    new_invite: &NewInvite,
    team_slug: &str,
) -> Result<(String, String), CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (_permissions, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), team_slug).await?;

    let invitation_selector = rand::rng().random::<[u8; 6]>();
    let invitation_selector_base64 = URL_SAFE_NO_PAD.encode(invitation_selector);
    let invitation_verifier = rand::rng().random::<[u8; 8]>();
    let invitation_verifier_hash = Sha256::digest(invitation_verifier);
    let invitation_verifier_hash_base64 = URL_SAFE_NO_PAD.encode(invitation_verifier_hash);
    let invitation_verifier_base64 = URL_SAFE_NO_PAD.encode(invitation_verifier);

    let roles = if new_invite.admin.is_some() {
        vec![
            types::public::Role::TeamManager,
            types::public::Role::Collaborator,
        ]
    } else {
        vec![types::public::Role::Collaborator]
    };

    queries::invitations::insert_invitation()
        .bind(
            &transaction,
            &team_id_num,
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
