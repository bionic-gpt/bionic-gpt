use super::super::{Authentication, CustomError};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
    Form,
};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct AcceptInvite {
    pub new_team_id: i32,
    pub team_id: i32,
    pub invite_id: i32,
}

pub async fn invite(
    Extension(_pool): Extension<Pool>,
    _current_user: Authentication,
    Form(accept_invite): Form<AcceptInvite>,
) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(
        &web_pages::routes::teams::Switch {
            team_id: accept_invite.team_id,
        }
        .to_string(),
    ))
}
