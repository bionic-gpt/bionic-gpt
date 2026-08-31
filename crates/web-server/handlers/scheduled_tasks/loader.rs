use crate::{config::Config, CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::{authz, Pool};
use web_pages::{routes::scheduled_tasks::Index, scheduled_tasks};

pub async fn loader(
    Index { team_id }: Index,
    locale: crate::locale::Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(_config): Extension<Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_chat_history() {
        return Err(CustomError::Authorization);
    }

    let tasks = db::queries::scheduled_tasks::list()
        .bind(&transaction)
        .all()
        .await?;
    Ok(Html(scheduled_tasks::page::page(
        rbac,
        team_id,
        tasks,
        locale.as_str(),
    )))
}
