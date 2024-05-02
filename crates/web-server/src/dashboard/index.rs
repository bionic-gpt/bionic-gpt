use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::audit_trail;
use db::Pool;
use web_pages::{dashboard, render_with_props, routes::dashboard::Index};

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let top_users = audit_trail::top_users().bind(&transaction).all().await?;

    let html = render_with_props(
        dashboard::index::Page,
        dashboard::index::PageProps {
            rbac,
            team_id,
            top_users,
        },
    );

    Ok(Html(html))
}
