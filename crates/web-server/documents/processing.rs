use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::documents;
use db::Pool;
use web_pages::routes::documents::Processing;

pub async fn row(
    Processing {
        team_id,
        document_id,
    }: Processing,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let document = documents::document()
        .bind(&transaction, &document_id)
        .one()
        .await?;

    let html = web_pages::documents::status::status(document, team_id, false);

    Ok(Html(html))
}
