mod index;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/app/team/:team_id/enterprise", get(index::index))
}
