use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::queries::{chats, chats_chunks, models, prompts};
use db::Pool;
use db::{authz, ModelType};
use web_pages::console::ChatWithChunks;
use web_pages::routes::prompts::Conversation;

pub async fn conversation(
    Conversation {
        team_id,
        conversation_id,
        prompt_id,
    }: Conversation,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let chats = chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;

    let mut chats_with_chunks = Vec::new();
    let mut lock_console = false;

    for chat in chats.into_iter() {
        // If any chat has not had a response yet, lock the console
        lock_console = chat.response.is_none();

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

    let prompt = prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let is_tts_disabled = models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

    let html = web_pages::prompts::conversation::page(
        team_id,
        rbac,
        chats_with_chunks,
        prompt,
        conversation_id,
        lock_console,
        is_tts_disabled,
    );

    Ok(Html(html))
}
