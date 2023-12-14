use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::{queries, Pool};
use ui_pages::api_keys;

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let api_keys = queries::api_keys::api_keys()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    Ok(Html(api_keys::index(api_keys::index::PageProps {
        team_id,
        is_sys_admin,
        api_keys,
        prompts,
    })))
}
