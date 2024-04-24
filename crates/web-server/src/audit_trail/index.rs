use super::super::{Authentication, CustomError};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::audit_trail::Index;

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let team_users = queries::teams::get_users()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let audits = queries::audit_trail::audit()
        .bind(
            &transaction,
            &None,
            &None,
            &None,
            &None,
            &(super::PAGE_SIZE + 1),
        )
        .all()
        .await?;

    let html = web_pages::render_with_props(
        web_pages::audit_trail::index::Page,
        web_pages::audit_trail::index::PageProps {
            team_id,
            rbac,
            team_users,
            audits,
            reset_search: true,
        },
    );

    Ok(Html(html))
}
