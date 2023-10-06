use crate::errors::CustomError;
use axum::response::Html;

pub async fn index() -> Result<Html<String>, CustomError> {
    Ok(Html(ui_components::index::index()))
}
