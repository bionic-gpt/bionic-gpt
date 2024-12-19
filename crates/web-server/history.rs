use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_post(search_action)
}

use super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::conversations;
use db::Pool;
use web_pages::{history, routes::history::Index};

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let history = conversations::history().bind(&transaction).all().await?;

    let html = history::index::page(rbac, team_id, history);

    Ok(Html(html))
}

use axum::Form;
use db::queries::models;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::history::Search;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SearchForm {
    #[validate(length(min = 1, message = "The search field is mandatory"))]
    pub search: String,
}

pub async fn search_action(
    Search { team_id }: Search,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(search): Form<SearchForm>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // We generate embeddings so we can search the history.
    let embeddings_model = models::get_system_embedding_model()
        .bind(&transaction)
        .one()
        .await?;

    let embeddings = embeddings_api::get_embeddings(
        &search.search,
        &embeddings_model.base_url,
        &embeddings_model.name,
        &embeddings_model.api_key,
    )
    .await
    .map_err(|e| CustomError::ExternalApi(e.to_string()));

    let history = if let Ok(embeddings) = embeddings {
        let results = db::search_history(&transaction, rbac.user_id, 10, embeddings).await?;
        tracing::info!("Retrieved {} search results", results.len());
        results
    } else {
        tracing::error!("Problem trying to get embeddings data");
        Default::default()
    };

    let html = history::results::page(rbac, team_id, history);

    Ok(Html(html))
}
