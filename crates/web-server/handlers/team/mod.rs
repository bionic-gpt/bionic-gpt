mod accept_invite;
mod create_invite;
mod delete_invite;
mod delete_member;
mod index;
mod set_name;
mod teams_popup;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_get(teams_popup::index)
        .typed_get(accept_invite::invite)
        .typed_post(create_invite::create_invite)
        .typed_post(delete_member::delete)
        .typed_post(delete_invite::delete)
        .typed_post(set_name::set_name)
}
