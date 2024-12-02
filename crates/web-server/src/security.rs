use crate::Jwt;

use super::CustomError;
use axum::extract::Extension;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::authz;
use db::Pool;
use web_pages::{render_with_props, security, security::routes::Index};

pub fn routes() -> Router {
    Router::new().typed_get(loader)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let html = render_with_props(
        security::SecurityPage,
        security::SecurityPageProps { team_id, rbac },
    );

    Ok(Html(html))
}
