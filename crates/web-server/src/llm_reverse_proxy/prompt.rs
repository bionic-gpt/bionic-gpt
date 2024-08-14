use super::Message;
use crate::errors::CustomError;
use db::queries::{chats, chats_chunks, prompts};
use db::{Chat, RelatedContext, Transaction};
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

// If we are getting called from the API we'll possible have a buch of chat messaages
// that's why chat is a Vec<Message>
// For the UI they'll be just one.
pub async fn execute_prompt(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    team_id: i32,
    conversation_id: Option<i64>,
    chat: Vec<Message>,
) -> Result<Vec<Message>, CustomError> {
    // Get the prompt
    let prompt = prompts::prompt()
        .bind(transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let question = if let Some(q) = chat.last() {
        q.content.clone()
    } else {
        "".to_string()
    };

    // Turn the users message into something the vector database can use
    let mut related_context = Default::default();
    if let (Some(embeddings_base_url), Some(embeddings_model)) =
        (prompt.embeddings_base_url, prompt.embeddings_model)
    {
        let embeddings = embeddings_api::get_embeddings(
            &question,
            &embeddings_base_url,
            &embeddings_model,
            &prompt.api_key,
        )
        .await
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?;

        tracing::info!(prompt.name);
        // Get related context
        related_context = db::get_related_context(
            transaction,
            prompt_id,
            team_id,
            prompt.max_chunks,
            embeddings,
        )
        .await?;
        tracing::info!("Retrieved {} chunks", related_context.len());
    }

    // Get the maximum required amount of chat history
    let chat_history = if let Some(conversation_id) = conversation_id {
        chats::chat_history()
            .bind(
                transaction,
                &conversation_id,
                &(prompt.max_history_items as i64),
            )
            .all()
            .await?
    } else {
        Default::default()
    };

    tracing::info!("Retrieved {} history items", chat_history.len());

    let trim_ratio = (prompt.trim_ratio as f32) / 100.0;

    let (messages, chunk_ids) = generate_prompt(
        prompt.model_context_size as usize,
        prompt.max_tokens as usize,
        trim_ratio,
        prompt.system_prompt,
        chat_history,
        chat,
        related_context,
    )
    .await;

    // Store the id's of the chunks we used for this particular chat
    // We assume, given a list that the last item is the one used for lookup
    if let Some(id) = conversation_id {
        for chunk_id in chunk_ids {
            chats_chunks::create_chunks_chats()
                .bind(transaction, &chunk_id, &id)
                .await?;
        }
    }

    let messages = messages
        .into_iter()
        .map(|msg| Message {
            role: msg.role,
            content: msg.content.unwrap_or("".to_string()),
        })
        .collect();

    Ok(messages)
}

async fn generate_prompt(
    model_context_size: usize,
    max_tokens: usize,
    trim_ratio: f32,
    system_prompt: Option<String>,
    mut history: Vec<Chat>,
    question: Vec<Message>,
    related_context: Vec<RelatedContext>,
) -> (Vec<ChatCompletionRequestMessage>, Vec<i32>) {
    let mut messages: Vec<ChatCompletionRequestMessage> = Default::default();
    // We need to remember which chunks are used in the chat.
    let mut chunk_ids: Vec<i32> = Default::default();

    let system_prompt = match (system_prompt, related_context.is_empty()) {
        (Some(prompt), false) => {
            Some(format!("{}\n\nContext information is below.\n--------------------\n{{context_str}}\n--------------------", prompt))
        }
        (Some(prompt), true) => {
            Some(prompt)
        }
        (None, false) => {
            Some("Context information is below.\n--------------------\n{{context_str}}\n--------------------".to_string())
        }
        (None, true) => {
            None
        }
    };

    // This is the space we have to fill
    let size_allowed = ((model_context_size - max_tokens) as f32 * trim_ratio) as usize;

    tracing::info!("Using context size of {}", size_allowed);

    let mut size_so_far = 0;

    // Add a system message if we have one
    if let Some(system_prompt) = &system_prompt {
        size_so_far = add_message(
            &mut messages,
            "system".to_string(),
            system_prompt.clone(),
            size_so_far,
            size_allowed,
        );
    }

    // Add the messages that have come from the UI or the API
    // This may already overflow the context!!
    for message in question.into_iter() {
        size_so_far = add_message(
            &mut messages,
            message.role,
            message.content,
            size_so_far,
            size_allowed,
        );
    }

    let mut related_context: Vec<&RelatedContext> = related_context.iter().rev().collect();
    let mut context_so_far: String = Default::default();

    // Keep adding history and context until meet the requirements of the prompt
    while size_so_far < size_allowed {
        // Add some relevant context
        if let Some(rel_context) = related_context.pop() {
            let size_rel_context = size_context(rel_context.chunk_text.to_string());

            if size_so_far + size_rel_context < size_allowed {
                context_so_far.push_str(&rel_context.chunk_text);
                chunk_ids.push(rel_context.chunk_id);
                context_so_far += "\n";
                if let Some(prompt) = &system_prompt {
                    let replaced = prompt.replace("{context_str}", &context_so_far);
                    messages[0].content = Some(replaced);
                }
                size_so_far += size_rel_context;
            }
        }

        // Add some history
        if let Some(hist) = history.pop() {
            // Add the histor in before the last message
            if let Some(top_message) = messages.pop() {
                size_so_far = add_message(
                    &mut messages,
                    "user".to_string(),
                    hist.user_request,
                    size_so_far,
                    size_allowed,
                );
                if let Some(response) = hist.response {
                    size_so_far = add_message(
                        &mut messages,
                        "assistant".to_string(),
                        response,
                        size_so_far,
                        size_allowed,
                    );
                }
                messages.push(top_message);
            }
        }

        if history.is_empty() && related_context.is_empty() {
            break;
        }
    }

    tracing::debug!("{:?}", &messages);

    (messages, chunk_ids)
}

fn size_context(context: String) -> usize {
    let request = ChatCompletionRequestMessage {
        role: "".to_string(),
        content: Some(context),
        name: None,
        function_call: None,
    };
    num_tokens_from_messages("gpt-4", &[request.clone()]).unwrap()
}

// Only add a message if the context doesn't overflow
fn add_message(
    messages: &mut Vec<ChatCompletionRequestMessage>,
    role: String,
    content: String,
    size_so_far: usize,
    size_allowed: usize,
) -> usize {
    let request = ChatCompletionRequestMessage {
        role,
        content: Some(content),
        name: None,
        function_call: None,
    };

    let size = num_tokens_from_messages("gpt-4", &[request.clone()]).unwrap();

    if (size + size_so_far) < size_allowed {
        messages.push(request);
        return size_so_far + size;
    }

    size_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_prompt() {
        let (messages, _chunk_ids) = generate_prompt(
            2048,
            1024,
            1.0,
            Some("You are a helpful asistant".to_string()),
            vec![create_prompt(
                "What time is it?".to_string(),
                "I don't know".to_string(),
            )],
            vec![Message {
                role: "user".to_string(),
                content: "How are you today?".to_string(),
            }],
            Default::default(),
        )
        .await;

        assert!(messages.len() == 4);

        assert!(messages[0].content == Some("You are a helpful asistant".to_string()));
        assert!(messages[3].content == Some("How are you today?".to_string()));
    }

    #[tokio::test]
    async fn test_generate_prompt_with_context() {
        let (messages, _chunk_ids) = generate_prompt(
            2048,
            1024,
            1.0,
            Some("You are a helpful asistant".to_string()),
            vec![create_prompt(
                "What time is it?".to_string(),
                "I don't know".to_string(),
            )],
            vec![Message {
                role: "user".to_string(),
                content: "How are you today?".to_string(),
            }],
            vec![RelatedContext {
                chunk_text: "This might help".to_string(),
                chunk_id: 0,
            }],
        )
        .await;

        assert!(messages.len() == 4);

        assert!(messages[0].content == Some("You are a helpful asistant\n\nContext information is below.\n--------------------\nThis might help\n\n--------------------".to_string()));
        assert!(messages[3].content == Some("How are you today?".to_string()));
    }

    #[tokio::test]
    async fn test_with_lots_of_context() {
        let (messages, _chunk_ids) = generate_prompt(
            2048,
            1024,
            1.0,
            Some("You are a helpful asistant".to_string()),
            vec![create_prompt(
                "What time is it?".to_string(),
                "I don't know".repeat(400),
            )],
            vec![Message {
                role: "user".to_string(),
                content: "How are you today?".to_string(),
            }],
            vec![
                RelatedContext {
                    chunk_text: "This might help".to_string(),
                    chunk_id: 0,
                },
                RelatedContext {
                    chunk_text: "word ".to_string(),
                    chunk_id: 0,
                },
                RelatedContext {
                    chunk_text: "test ".to_string(),
                    chunk_id: 0,
                },
                RelatedContext {
                    chunk_text: "name ".to_string(),
                    chunk_id: 0,
                },
            ],
        )
        .await;

        let size_so_far = num_tokens_from_messages("gpt-4", &messages).unwrap();

        assert!(size_so_far < 1024);
    }

    fn create_prompt(question: String, answer: String) -> Chat {
        Chat {
            id: 0,
            conversation_id: 0,
            user_request: question,
            prompt: "todo!()".to_string(),
            prompt_id: 0,
            model_name: "ggml".to_string(),
            response: Some(answer),
            created_at: time::OffsetDateTime::UNIX_EPOCH,
            updated_at: time::OffsetDateTime::UNIX_EPOCH,
        }
    }
}
