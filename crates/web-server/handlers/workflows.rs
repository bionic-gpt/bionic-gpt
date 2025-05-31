use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::workflows::{Delete, Index, Upsert, View};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_get(view)
        .typed_post(upsert_action)
        .typed_post(delete_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // For now, we don't have any workflows in the database yet
    // This is just a mockup screen as mentioned in the requirements
    let html = web_pages::workflows::index::page(rbac, team_id);

    Ok(Html(html))
}

pub async fn view(
    View { team_id, id }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Get sample workflows and find the one with matching ID
    let workflows = web_pages::workflows::workflow_cards::get_sample_workflows();
    let workflow = workflows.into_iter().find(|w| w.id == id);

    let html = web_pages::workflows::view::view(team_id, rbac, workflow);

    Ok(Html(html))
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct WorkflowForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub description: Option<String>,
    pub trigger_type: String,
}

pub async fn upsert_action(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(workflow_form): Form<WorkflowForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // For now, we don't actually save anything to the database
    // This is just a mockup as mentioned in the requirements
    if workflow_form.validate().is_ok() {
        // TODO: Implement actual workflow creation when database schema is ready
        tracing::info!("Would create workflow: {:?}", workflow_form);
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::workflows::Index { team_id }.to_string(),
        "Workflow Created (Mockup)",
    )
}

pub async fn delete_action(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // For now, we don't actually delete anything from the database
    // This is just a mockup as mentioned in the requirements
    tracing::info!("Would delete workflow with id: {}", id);

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::workflows::Index { team_id }.to_string(),
        "Workflow Deleted (Mockup)",
    )
}
