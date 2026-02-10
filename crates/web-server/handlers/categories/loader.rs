use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::{queries, Pool};
use web_pages::routes::categories::Index;

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let categories = queries::categories::categories()
        .bind(&transaction)
        .all()
        .await?;

    let html = web_pages::categories::page::page(team_id, rbac, categories);

    Ok(Html(html))
}
