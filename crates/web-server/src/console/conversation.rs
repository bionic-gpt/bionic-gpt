use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::queries::{chats, chats_chunks, conversations, models, prompts};
use db::Pool;
use db::{authz, ModelType};
use web_pages::console::ChatWithChunks;
use web_pages::{console, render_with_props, routes::console::Conversation};

pub async fn conversation(
    Conversation {
        team_id,
        conversation_id,
    }: Conversation,
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

    let is_tts_disabled = models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

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

    let prompts = prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Model)
        .all()
        .await?;

    let prompt = prompts::prompt()
        .bind(&transaction, &prompts.first().unwrap().id, &team_id)
        .one()
        .await?;

    let html = render_with_props(
        console::index::Page,
        console::index::PageProps {
            team_id,
            rbac,
            conversation_id,
            chats_with_chunks,
            prompts,
            prompt,
            history,
            lock_console,
            is_tts_disabled,
        },
    );

    Ok(Html(html))
}
