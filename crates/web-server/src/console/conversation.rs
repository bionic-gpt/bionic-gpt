use super::super::{Authentication, CustomError};
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries::{chats, chats_chunks, conversations, prompts};
use db::Pool;
use web_pages::console;
use web_pages::console::ChatWithChunks;

pub async fn conversation(
    Path((team_id, conversation_id)): Path<(i32, i64)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let history = conversations::history().bind(&transaction).all().await?;

    let chats = chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;

    let mut chats_with_chunks = Vec::new();
    let mut lock_console = false;

    for chat in chats.into_iter() {
        // If any chat has not had a response yet, lock the console
        if chat.response.is_none() {
            lock_console = true;
        }

        // Get all chunks for each chat
        let chunks_chats = chats_chunks::chunks_chats()
            .bind(&transaction, &chat.id)
            .all()
            .await?;
        let chat_with_chunks = ChatWithChunks {
            chat,
            chunks: chunks_chats,
        };
        chats_with_chunks.push(chat_with_chunks);
    }

    let prompts = prompts::prompts()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    Ok(Html(console::index(console::index::PageProps {
        team_id,
        rbac,
        conversation_id,
        chats_with_chunks,
        prompts,
        history,
        lock_console,
    })))
}
