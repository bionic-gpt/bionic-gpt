use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries::documents;
use db::Pool;

pub async fn status(
    Path(document_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    authz::set_row_level_security_user_id(&transaction, current_user.sub).await?;

    let document = documents::document()
        .bind(&transaction, &document_id)
        .one()
        .await?;

    Ok(Html(ui_pages::documents::status(document)))
}
