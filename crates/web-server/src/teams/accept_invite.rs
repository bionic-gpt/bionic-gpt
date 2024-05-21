use super::super::{Authentication, CustomError};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
    Form,
};
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::teams::AcceptInvite;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct AcceptInviteForm {
    pub new_team_id: i32,
    pub team_id: i32,
    pub invite_id: i32,
}

pub async fn accept_invite(
    AcceptInvite {}: AcceptInvite,
    _current_user: Authentication,
    Extension(_pool): Extension<Pool>,
    Form(accept_invite): Form<AcceptInviteForm>,
) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(
        &web_pages::routes::teams::Switch {
            team_id: accept_invite.team_id,
        }
        .to_string(),
    ))
}
