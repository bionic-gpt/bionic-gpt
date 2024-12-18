use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::{datasets, models};
use db::{ModelType, Pool};
use web_pages::routes::datasets::Index;

pub async fn index(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let datasets = datasets::datasets().bind(&transaction).all().await?;

    let models = models::models()
        .bind(&transaction, &ModelType::Embeddings)
        .all()
        .await?;

    let can_set_visibility_to_company = rbac.is_sys_admin;

    let html = web_pages::datasets::index::page(
        rbac,
        team_id,
        datasets,
        models,
        can_set_visibility_to_company,
    );

    Ok(Html(html))
}
