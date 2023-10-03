use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::{Html, IntoResponse};
use axum::Form;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

pub async fn new(Path(team_id): Path<i32>) -> Result<Html<String>, CustomError> {
    Ok(Html(ui_components::models::form::form(team_id, None)))
}

pub async fn edit(
    Path((team_id, model_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let model = queries::models::model()
        .bind(&transaction, &model_id)
        .one()
        .await?;

    Ok(Html(ui_components::models::form::form(
        team_id,
        Some(model),
    )))
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ModelForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The prompt is mandatory"))]
    pub base_url: String,
    pub api_key: Option<String>,
    pub billion_parameters: i32,
    pub context_size: i32,
}

pub async fn upsert(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(model_form): Form<ModelForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    match (model_form.validate(), model_form.id) {
        (Ok(_), Some(id)) => {
            // The form is valid save to the database
            queries::models::update()
                .bind(
                    &transaction,
                    &model_form.name,
                    &model_form.base_url,
                    &model_form.api_key,
                    &model_form.billion_parameters,
                    &model_form.context_size,
                    &id,
                )
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_components::routes::models::index_route(team_id),
                "Model Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            // The form is valid save to the database
            queries::models::insert()
                .bind(
                    &transaction,
                    &model_form.name,
                    &team_id,
                    &model_form.base_url,
                    &model_form.api_key,
                    &model_form.billion_parameters,
                    &model_form.context_size,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &ui_components::routes::models::index_route(team_id),
                "Model Created",
            )
            .into_response())
        }
        (Err(_), _) => {
            let html = ui_components::models::form::form(team_id, None);
            Ok(html.into_response())
        }
    }
}
