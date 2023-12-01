mod index;
mod profile_popup;
mod set_details;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:team_id/profile", get(index::index))
        .route(
            "/app/team/:team_id/profile_popup",
            get(profile_popup::index),
        )
        .route(
            "/app/team/:team_id/set_details",
            post(set_details::set_details),
        )
}

pub fn index_route(team_id: i32) -> String {
    format!("/app/team/{}/profile", team_id)
}
