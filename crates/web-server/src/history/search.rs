use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use axum::Form;
use db::authz;
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
    Form(_search): Form<SearchForm>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let html = render_with_props(
        history::results::Page,
        history::results::PageProps { team_id, rbac },
    );

    Ok(Html(html))
}
