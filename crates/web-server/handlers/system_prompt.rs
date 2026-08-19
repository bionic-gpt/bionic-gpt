use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::system_prompt::{Index, Update};

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_post(update_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let setting = queries::runtime_settings::default_system_prompt()
        .bind(&transaction)
        .one()
        .await?;
    let skill_summaries = queries::skills::visible_skill_summaries()
        .bind(&transaction)
        .all()
        .await?;
    let runtime_additions =
        tool_runtime::skills::available_skills_prompt_section_with_custom(skill_summaries);

    let html = web_pages::system_prompt::page::page(team_id, rbac, setting, runtime_additions);

    Ok(Html(html))
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SystemPromptForm {
    #[validate(length(min = 1, message = "The system prompt is mandatory"))]
    pub value: String,
}

pub async fn update_action(
    Update { team_id }: Update,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<SystemPromptForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    if form.validate().is_err() {
        return Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::system_prompt::Index { team_id }.to_string(),
            "System Prompt Validation Error",
        )
        .into_response());
    }

    queries::runtime_settings::update_default_system_prompt()
        .bind(&transaction, &form.value)
        .await?;
    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::system_prompt::Index { team_id }.to_string(),
        "System Prompt Updated",
    )
    .into_response())
}
