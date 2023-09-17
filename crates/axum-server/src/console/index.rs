use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::{chats, prompts};
use db::Pool;

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let chats = chats::chats().bind(&transaction).all().await?;
    let prompts = prompts::prompts()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let send_action = ui_components::routes::console::send_message_route(team_id);
    let update_response = ui_components::routes::console::update_response_route(team_id);

    Ok(Html(ui_components::console::index(
        team_id,
        send_action,
        update_response,
        chats,
        prompts,
    )))
}
