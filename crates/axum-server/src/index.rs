use axum::response::{IntoResponse, Redirect};

pub async fn index() -> impl IntoResponse {
    Redirect::permanent("/auth/sign_in")
}
