use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::conversations;
use db::Pool;
use web_pages::{history, routes::history::Index};

pub async fn index(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let history = conversations::history().bind(&transaction).all().await?;

    let html = history::index::page(rbac, team_id, history);

    Ok(Html(html))
}
