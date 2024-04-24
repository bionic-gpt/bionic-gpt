use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::{audit_trail, models};
use db::{ModelType, Pool};
use web_pages::routes::models::Index;

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let mut models = models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;
    models.append(
        &mut models::models()
            .bind(&transaction, &ModelType::Embeddings)
            .all()
            .await?,
    );

    let top_users = audit_trail::top_users().bind(&transaction).all().await?;

    let html = web_pages::render_with_props(
        web_pages::models::index::Page,
        web_pages::models::index::PageProps {
            team_id,
            rbac,
            models,
            top_users,
        },
    );

    Ok(Html(html))
}
