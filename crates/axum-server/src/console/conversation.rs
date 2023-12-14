use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::{chats, conversations, prompts};
use db::Pool;
use ui_pages::console;

pub async fn conversation(
    Path((team_id, conversation_id)): Path<(i32, i64)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let history = conversations::history().bind(&transaction).all().await?;

    let chats = chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;
    let prompts = prompts::prompts()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    // If one of the chats is not processed yet then set a lock_console flag
    // Otherwise the user can issue multiple requests
    let lock_console = chats.iter().any(|chat| chat.response.is_none());

    Ok(Html(console::index(console::index::PageProps {
        team_id,
        is_sys_admin,
        conversation_id,
        chats,
        prompts,
        history,
        lock_console,
    })))
}
