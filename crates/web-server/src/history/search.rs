use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use axum::Form;
use db::authz;
use db::queries::models;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::{history, render_with_props, routes::history::Search};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SearchForm {
    #[validate(length(min = 1, message = "The search field is mandatory"))]
    pub search: String,
}

pub async fn search(
    Search { team_id }: Search,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(search): Form<SearchForm>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // We generate embeddings so we can search the history.
    let embeddings_model = models::get_system_model().bind(&transaction).one().await?;

    let embeddings = embeddings_api::get_embeddings(
        &search.search,
        &embeddings_model.base_url,
        &embeddings_model.name,
        &embeddings_model.api_key,
    )
    .await
    .map_err(|e| CustomError::ExternalApi(e.to_string()));

    let results = if let Ok(embeddings) = embeddings {
        let results = db::search_history(&transaction, rbac.user_id, 10, embeddings).await?;
        tracing::info!("Retrieved {} search results", results.len());
        results
    } else {
        tracing::error!("Problem trying to get embeddings data");
        Default::default()
    };

    let html = render_with_props(
        history::results::Page,
        history::results::PageProps {
            team_id,
            rbac,
            results,
        },
    );

    Ok(Html(html))
}
