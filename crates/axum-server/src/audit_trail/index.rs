use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use db::queries;
use db::Pool;

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

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

    Ok(Html(ui_pages::audit_trail::index::index(
        ui_pages::audit_trail::index::PageProps {
            team_id,
            is_sys_admin,
            team_users,
            audits,
            reset_search: true,
        },
    )))
}
