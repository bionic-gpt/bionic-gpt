use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::{audit_trail, models};
use db::{ModelType, Pool};

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

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

    Ok(Html(ui_pages::models::index(
        ui_pages::models::index::PageProps {
            team_id,
            is_sys_admin,
            models,
            top_users,
        },
    )))
}
