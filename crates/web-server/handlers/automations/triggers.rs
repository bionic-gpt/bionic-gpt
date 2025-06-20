use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::{Html, IntoResponse}};
use axum_extra::extract::Form;
use db::{authz, queries, Pool};
use serde::Deserialize;
use web_pages::routes::automations::{AddCronTrigger, ManageTriggers, RemoveCronTrigger};

pub async fn manage_triggers(
    ManageTriggers { team_id, prompt_id }: ManageTriggers,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let triggers = queries::automation_triggers::cron_triggers_by_prompt()
        .bind(&transaction, &prompt_id)
        .all()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let html = web_pages::automations::triggers::page(team_id, prompt_id, prompt.name, rbac, triggers);

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct CronForm {
    minute: String,
    hour: String,
    day: String,
    month: String,
    weekday: String,
}

pub async fn add_cron_trigger(
    AddCronTrigger { team_id, prompt_id }: AddCronTrigger,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<CronForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let cron = format!("{} {} {} {} {}", form.minute, form.hour, form.day, form.month, form.weekday);

    queries::automation_triggers::insert_cron_trigger()
        .bind(&transaction, &prompt_id, &cron)
        .one()
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::automations::ManageTriggers { team_id, prompt_id }.to_string(),
        "Cron trigger added",
    ).into_response())
}

pub async fn remove_cron_trigger(
    RemoveCronTrigger { team_id, prompt_id, trigger_id }: RemoveCronTrigger,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::automation_triggers::delete_cron_trigger()
        .bind(&transaction, &trigger_id, &prompt_id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::automations::ManageTriggers { team_id, prompt_id }.to_string(),
        "Cron trigger removed",
    ).into_response())
}
