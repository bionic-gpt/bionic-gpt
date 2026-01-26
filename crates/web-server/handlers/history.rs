use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_post(search_action)
}

use crate::{locale::Locale, CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::Pool;
use web_pages::{history, routes::history::Index};

pub async fn loader(
    Index { team_id }: Index,
    locale: Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    let i18n = db::i18n::global();
    i18n.ensure_locale("en").await;
    if locale.as_str() != "en" {
        i18n.ensure_locale(locale.as_str()).await;
    }

    let history = db::queries::history::history()
        .bind(&transaction)
        .all()
        .await?;

    let html = history::page::page(rbac, team_id, history, locale.as_str());

    Ok(Html(html))
}

use axum::Form;
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
    locale: Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(search): Form<SearchForm>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    let i18n = db::i18n::global();
    i18n.ensure_locale("en").await;
    if locale.as_str() != "en" {
        i18n.ensure_locale(locale.as_str()).await;
    }

    // Use SQL-based search instead of embeddings
    let history = db::queries::history::search_history()
        .bind(&transaction, &rbac.user_id, &search.search, &10)
        .all()
        .await?;

    tracing::info!("Retrieved {} search results", history.len());

    let html = history::results::page(rbac, team_id, history, locale.as_str());

    Ok(Html(html))
}
