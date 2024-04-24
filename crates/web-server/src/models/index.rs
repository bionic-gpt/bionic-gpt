use super::super::{Authentication, CustomError};
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries::{audit_trail, models};
use db::{ModelType, Pool};

pub async fn index(
    Path(team_id): Path<i32>,
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

    Ok(Html(web_pages::models::index(
        web_pages::models::index::PageProps {
            team_id,
            rbac,
            models,
            top_users,
        },
    )))
}
