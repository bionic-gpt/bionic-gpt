use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::oauth_clients::{Index, New};

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let oauth_clients = queries::oauth_clients::oauth_clients()
        .bind(&transaction)
        .all()
        .await?;

    let html = web_pages::oauth_clients::page::page(team_id, rbac, oauth_clients);

    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let oauth_client = web_pages::oauth_clients::upsert::OauthClientForm::default();
    let html = web_pages::oauth_clients::upsert::page(team_id, rbac, oauth_client);

    Ok(Html(html))
}
