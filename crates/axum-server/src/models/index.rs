use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::models;
use db::{ModelType, Pool};

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

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

    Ok(Html(ui_components::models::index(team_id, models)))
}
