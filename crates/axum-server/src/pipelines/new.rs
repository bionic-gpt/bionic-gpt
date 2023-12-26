use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::authz;
use db::queries::document_pipelines;
use db::Pool;
use serde::Deserialize;
use ui_pages::routes::document_pipelines::index_route;
use validator::Validate;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewForm {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub dataset_id: i32,
}

pub async fn new(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(new_pipeline): Form<NewForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, current_user.sub, team_id).await?;

    if new_pipeline.validate().is_ok() {
        let api_key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        document_pipelines::insert()
            .bind(
                &transaction,
                &new_pipeline.dataset_id,
                &rbac.user_id,
                &team_id,
                &new_pipeline.name,
                &api_key,
            )
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&index_route(team_id), "Pipeline Created")
}
