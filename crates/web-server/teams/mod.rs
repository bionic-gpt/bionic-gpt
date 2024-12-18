pub mod accept_invite;
pub mod delete;
pub mod new_team;
pub mod switch;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(switch::switch)
        .typed_post(new_team::new_team)
        .typed_post(accept_invite::accept_invite)
        .typed_post(delete::delete)
}
