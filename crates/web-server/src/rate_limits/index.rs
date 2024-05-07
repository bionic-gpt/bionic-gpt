use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::{authz, queries, ModelType, Pool};
use web_pages::{rate_limits, render_with_props, routes::rate_limits::Index};

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let rate_limits = queries::rate_limits::rate_limits()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;

    let html = render_with_props(
        rate_limits::index::Page,
        rate_limits::index::PageProps {
            rbac,
            team_id,
            rate_limits,
            models,
        },
    );

    Ok(Html(html))
}
