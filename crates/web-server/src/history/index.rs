use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::conversations;
use db::Pool;
use web_pages::{history, render_with_props, routes::history::Index};

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let history = conversations::history().bind(&transaction).all().await?;

    let html = render_with_props(
        history::index::Page,
        history::index::PageProps {
            team_id,
            rbac,
            history,
        },
    );

    Ok(Html(html))
}
