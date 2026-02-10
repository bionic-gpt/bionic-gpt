use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::{queries, Pool};
use web_pages::routes::projects::{Index, View};

pub async fn index(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<crate::config::Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let team_slug = team_id;
    let (rbac, _team_id) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_slug).await?;

    if !rbac.can_manage_projects() {
        return Err(CustomError::Authorization);
    }

    let projects = queries::projects::projects()
        .bind(&transaction)
        .all()
        .await?;

    let can_set_visibility_to_company = !config.saas && rbac.is_sys_admin;

    let html =
        web_pages::projects::page::page(team_slug, rbac, projects, can_set_visibility_to_company);

    Ok(Html(html))
}

pub async fn view(
    View {
        team_id,
        project_id,
    }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<crate::config::Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let team_slug = team_id;
    let (rbac, _team_id) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_slug).await?;

    if !rbac.can_manage_projects() {
        return Err(CustomError::Authorization);
    }

    let project = queries::projects::project()
        .bind(&transaction, &project_id)
        .one()
        .await?;

    let histories = queries::history::project_history()
        .bind(&transaction, &project_id)
        .all()
        .await?;

    let documents = queries::documents::documents()
        .bind(&transaction, &project.dataset_id)
        .all()
        .await?;

    let can_set_visibility_to_company = !config.saas && rbac.is_sys_admin;

    let html = web_pages::projects::view::page(
        team_slug,
        rbac,
        project,
        histories,
        documents,
        can_set_visibility_to_company,
    );

    Ok(Html(html))
}
