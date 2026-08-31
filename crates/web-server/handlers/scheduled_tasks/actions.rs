use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::IntoResponse, Form};
use chrono::Utc;
use db::authz;
use db::Pool;
use serde::Deserialize;
use tool_runtime::scheduled_tasks::next_run_at;
use web_pages::routes::scheduled_tasks::{Delete, Update};

#[derive(Debug, Deserialize)]
pub struct UpdateForm {
    pub id: i64,
    pub name: String,
    pub prompt: String,
    pub cron: String,
    pub timezone: String,
    pub enabled: bool,
}

pub async fn action_update(
    Update { team_id }: Update,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<UpdateForm>,
) -> Result<impl IntoResponse, CustomError> {
    if form.name.trim().is_empty() || form.prompt.trim().is_empty() {
        return redirect(&team_id, "Name and prompt are required");
    }
    let next_run_at = match next_run_at(&form.cron, &form.timezone, Utc::now()) {
        Ok(value) => value.fixed_offset(),
        Err(error) => return redirect(&team_id, error),
    };

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_chat_history() {
        return Err(CustomError::Authorization);
    }
    db::queries::scheduled_tasks::update()
        .bind(
            &transaction,
            &form.name,
            &form.prompt,
            &form.cron,
            &form.timezone,
            &form.enabled,
            &next_run_at,
            &form.id,
        )
        .one()
        .await?;
    transaction.commit().await?;
    redirect(&team_id, "Scheduled task updated")
}

pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_chat_history() {
        return Err(CustomError::Authorization);
    }
    db::queries::scheduled_tasks::delete()
        .bind(&transaction, &id)
        .await?;
    transaction.commit().await?;
    redirect(&team_id, "Scheduled task deleted")
}

fn redirect(
    team_id: &str,
    message: impl Into<String>,
) -> Result<axum::response::Response, CustomError> {
    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::scheduled_tasks::Index {
            team_id: team_id.to_string(),
        }
        .to_string(),
        message,
    )
    .into_response())
}
