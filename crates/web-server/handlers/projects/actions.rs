use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::types::public::ChunkingStrategy;
use db::{Pool, Visibility};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    routes::projects::{Delete, StartChat, Upsert, View},
    string_to_visibility,
};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ProjectForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub instructions: String,
    pub visibility: String,
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<crate::config::Config>,
    current_user: Jwt,
    Form(form): Form<ProjectForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_projects() {
        return Err(CustomError::Authorization);
    }

    let mut visibility = string_to_visibility(&form.visibility);

    if visibility == Visibility::Company && config.saas {
        visibility = Visibility::Team;
    }
    if visibility == Visibility::Company && !rbac.is_sys_admin {
        visibility = Visibility::Team;
    }

    match (form.validate(), form.id) {
        (Ok(_), Some(id)) => {
            queries::projects::update()
                .bind(
                    &transaction,
                    &form.name,
                    &form.instructions,
                    &visibility,
                    &id,
                )
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &View {
                    team_id,
                    project_id: id,
                }
                .to_string(),
                "Project Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            let embeddings_model = queries::models::get_system_embedding_model()
                .bind(&transaction)
                .one()
                .await?;

            let dataset_id = queries::datasets::insert_project()
                .bind(
                    &transaction,
                    &team_id,
                    &form.name,
                    &embeddings_model.id,
                    &ChunkingStrategy::ByTitle,
                    &500,
                    &1000,
                    &true,
                    &visibility,
                )
                .one()
                .await?;

            let project_id = queries::projects::insert()
                .bind(
                    &transaction,
                    &team_id,
                    &dataset_id,
                    &form.name,
                    &form.instructions,
                    &visibility,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &View {
                    team_id,
                    project_id,
                }
                .to_string(),
                "Project Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::projects::Index { team_id }.to_string(),
            "Project Validation Error",
        )
        .into_response()),
    }
}

pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_projects() {
        return Err(CustomError::Authorization);
    }

    let project = queries::projects::project()
        .bind(&transaction, &id)
        .one()
        .await?;

    queries::projects::delete().bind(&transaction, &id).await?;
    queries::datasets::delete()
        .bind(&transaction, &project.dataset_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::projects::Index { team_id }.to_string(),
        "Project Deleted",
    )
}

pub async fn action_start_chat(
    StartChat {
        team_id,
        project_id,
    }: StartChat,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_projects() {
        return Err(CustomError::Authorization);
    }

    queries::projects::project()
        .bind(&transaction, &project_id)
        .one()
        .await?;

    let conversation_id = queries::conversations::create_project_conversation()
        .bind(&transaction, &team_id, &project_id)
        .one()
        .await?;

    transaction.commit().await?;

    crate::layout::redirect(
        &web_pages::routes::console::Conversation {
            team_id,
            conversation_id,
        }
        .to_string(),
    )
}
