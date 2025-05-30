use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Multipart},
    response::IntoResponse,
};
use db::{authz, PromptType};
use db::{
    queries::{attachments, chats, conversations, prompts},
    ChatRole,
};
use db::{ChatStatus, Pool};
use object_storage;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::console::SendMessage;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
    pub conversation_id: Option<i64>,
    pub prompt_id: i32,
}

pub async fn send_message(
    SendMessage { team_id }: SendMessage,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    // Initialize variables to store form data
    let mut message_text = String::new();
    let mut conversation_id: Option<i64> = None;
    let mut prompt_id: Option<i32> = None;
    // Store file information and data for later processing
    let mut files_info: Vec<(String, String, Vec<u8>, usize)> = Vec::new();

    // Process multipart form
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "message" => {
                message_text = field.text().await?;
            }
            "conversation_id" => {
                if let Ok(text) = field.text().await {
                    if !text.is_empty() {
                        conversation_id = Some(text.parse::<i64>().unwrap_or_default());
                    }
                }
            }
            "prompt_id" => {
                if let Ok(text) = field.text().await {
                    prompt_id = Some(text.parse::<i32>().unwrap_or_default());
                }
            }
            "attachments" => {
                if let Some(file_name) = field.file_name() {
                    // Clone the file_name to avoid borrowing issues
                    let file_name = file_name.to_string();
                    let content_type = field
                        .content_type()
                        .unwrap_or("application/octet-stream")
                        .to_string();
                    let data = field.bytes().await?;
                    let size = data.len();

                    // Log file information
                    tracing::info!(
                        "Received file: name={}, type={}, size={}",
                        file_name,
                        content_type,
                        size
                    );

                    // Store file info and data for later processing
                    files_info.push((file_name, content_type, data.to_vec(), size));
                }
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    // Validate required fields
    if message_text.is_empty() || prompt_id.is_none() {
        return crate::layout::redirect(&web_pages::routes::console::Index { team_id }.to_string());
    }

    // Create a Message struct for compatibility with existing code
    let message = Message {
        message: message_text,
        conversation_id,
        prompt_id: prompt_id.unwrap(),
    };

    // Validate the message
    if message.validate().is_ok() {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

        let conversation_id = if let Some(conversation_id) = message.conversation_id {
            conversation_id
        } else {
            conversations::create_conversation()
                .bind(&transaction, &team_id, &None)
                .one()
                .await?
        };

        // Store the prompt, ready for the front end webcomponent to pickup
        let chat_id = chats::new_chat()
            .bind(
                &transaction,
                &conversation_id,
                &message.prompt_id,
                &None::<String>,
                &None::<String>,
                &message.message,
                &ChatRole::User,
                &ChatStatus::Pending,
            )
            .one()
            .await?;

        // Handle attachments if any
        handle_attachments(
            &transaction,
            pool.clone(),
            &chat_id,
            team_id,
            rbac.user_id,
            &files_info,
        )
        .await?;

        let prompt = prompts::prompt()
            .bind(&transaction, &message.prompt_id, &team_id)
            .one()
            .await?;

        transaction.commit().await?;

        if prompt.prompt_type == PromptType::Assistant {
            crate::layout::redirect(
                &web_pages::routes::prompts::Conversation {
                    team_id,
                    conversation_id,
                    prompt_id: prompt.id,
                }
                .to_string(),
            )
        } else {
            crate::layout::redirect(
                &web_pages::routes::console::Conversation {
                    team_id,
                    conversation_id,
                }
                .to_string(),
            )
        }
    } else {
        crate::layout::redirect(&web_pages::routes::console::Index { team_id }.to_string())
    }
}

/// Handles the processing and storage of file attachments.
///
/// # Arguments
///
/// * `transaction` - A reference to the current database transaction.
/// * `pool` - The database connection pool.
/// * `chat_id` - The ID of the chat to link attachments to.
/// * `team_id` - The ID of the team the attachments belong to.
/// * `user_id` - The ID of the user uploading the attachments.
/// * `files_info` - A vector of tuples containing file information (name, content_type, data, size).
///
/// # Returns
///
/// * `Result<(), CustomError>` - Returns `Ok` if successful, or a `CustomError` otherwise.
async fn handle_attachments(
    transaction: &db::Transaction<'_>,
    pool: Pool,
    chat_id: &i32,
    team_id: i32,
    user_id: i32,
    files_info: &[(String, String, Vec<u8>, usize)],
) -> Result<(), CustomError> {
    for (file_name, _content_type, file_data, _size) in files_info {
        // Upload the file to object storage
        match object_storage::upload(pool.clone(), user_id, team_id, file_name, file_data).await {
            Ok(object_id) => {
                // Link the object to the chat
                attachments::insert()
                    .bind(transaction, chat_id, &object_id)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to link attachment: {}", e);
                        CustomError::Database(e.to_string())
                    })?;

                tracing::info!(
                    "Attachment stored: file={}, object_id={}, chat_id={}",
                    file_name,
                    object_id,
                    chat_id
                );
            }
            Err(e) => {
                tracing::error!("Failed to upload attachment: {}", e);
                return Err(CustomError::ExternalApi(format!(
                    "Failed to upload attachment: {}",
                    e
                )));
            }
        }
    }

    Ok(())
}
