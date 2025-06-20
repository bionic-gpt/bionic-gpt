use axum::{
    extract::{Extension, Form},
    response::Html,
    Router,
};
use axum_extra::routing::RouterExt;
use db::authz;
use db::queries::{self, datasets, models};
use db::types::public::ChunkingStrategy;
use db::{ModelType, Pool, Visibility};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    routes::datasets::{Delete, Index, Upsert},
    string_to_visibility,
};
use crate::config::Config;
use crate::{CustomError, Jwt};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(upsert_action)
        .typed_post(delete_action)
}

// Index function
pub async fn loader(
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

    let html = web_pages::datasets::page::page(
        rbac,
        team_id,
        datasets,
        models,
        can_set_visibility_to_company,
    );

    Ok(Html(html))
}
