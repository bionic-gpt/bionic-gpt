use crate::config::Config;

use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::licences;
use db::Pool;
use web_pages::{licence, render_with_props, routes::licence::Index};

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let days_ago = licences::days_since_first_registration()
        .bind(&transaction)
        .one()
        .await?;

    let remaining_days = 60 - days_ago;

    let html = render_with_props(
        licence::index::Page,
        licence::index::PageProps {
            rbac,
            team_id,
            version: config.version,
            remaining_days,
        },
    );

    Ok(Html(html))
}
