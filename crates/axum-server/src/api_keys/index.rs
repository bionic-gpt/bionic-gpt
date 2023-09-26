use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::{queries, Pool};

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    crate::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let api_keys = queries::api_keys::api_keys()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    Ok(Html(ui_components::api_keys::index(
        api_keys, prompts, team_id,
    )))
}
