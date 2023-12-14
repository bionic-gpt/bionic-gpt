use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::documents;
use db::Pool;

pub async fn status(
    Path(document_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let document = documents::document()
        .bind(&transaction, &document_id)
        .one()
        .await?;

    Ok(Html(ui_pages::documents::status(document)))
}
