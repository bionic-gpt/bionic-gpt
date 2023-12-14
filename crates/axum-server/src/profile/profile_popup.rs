use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries;
use db::Pool;

pub async fn index(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let user = queries::users::user()
        .bind(&transaction, &current_user.user_id)
        .one()
        .await?;

    Ok(Html(ui_pages::profile_popup::profile_popup(user, team_id)))
}
