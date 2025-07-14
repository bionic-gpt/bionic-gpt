use crate::{config::Config, CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, Pool};
use web_pages::{licence, routes::licence::Index};

pub fn routes() -> Router {
    Router::new().typed_get(loader)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(config): Extension<Config>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let callback_url = config.oauth2_redirect_uri();
    let html = licence::page::page(team_id, rbac, callback_url);

    Ok(Html(html))
}
