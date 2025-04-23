use crate::errors::CustomError;
use crate::token_count::{token_count, token_count_from_string};
use db::queries::{chats_chunks, prompts};
use db::{RelatedContext, Transaction};
use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole};

// If we are getting called from the API we'll possible have a buch of chat messaages
// that's why chat is a Vec<Message>
// For the UI they'll be just one.
pub async fn execute_prompt(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    team_id: i32,
    conversation_id: Option<i64>,
    chat_history: Vec<ChatCompletionMessage>,
) -> Result<Vec<ChatCompletionMessage>, CustomError> {
    // Get the prompt
    let prompt = prompts::prompt()
        .bind(transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let question = if let Some(q) = chat_history.last() {
        q.content.clone().unwrap_or("".to_string())
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
            prompt.model_context_size,
            &prompt.api_key,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                "Problem getting embeddings {} {}",
                embeddings_base_url,
                embeddings_model
            );
            CustomError::ExternalApi(e.to_string())
        })?;

        tracing::info!(prompt.name);
        // Get related context
        related_context =
            db::get_related_context(transaction, prompt_id, prompt.max_chunks, embeddings).await?;
        tracing::info!("Retrieved {} chunks", related_context.len());
    }

    tracing::info!("Retrieved {} history items", chat_history.len());

    let trim_ratio = (prompt.trim_ratio as f32) / 100.0;

    let (messages, chunk_ids) = generate_prompt(
        prompt.model_context_size as usize,
        prompt.max_tokens as usize,
        trim_ratio,
        prompt.system_prompt,
        chat_history,
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

    Ok(messages)
}

pub async fn generate_prompt(
    model_context_size: usize,
    max_tokens: usize,
    trim_ratio: f32,
    system_prompt: Option<String>,
    history: Vec<ChatCompletionMessage>,
    related_context: Vec<RelatedContext>,
) -> (Vec<ChatCompletionMessage>, Vec<i32>) {
    let mut messages: Vec<ChatCompletionMessage> = Default::default();
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
    let size_allowed = if max_tokens < model_context_size {
        ((model_context_size - max_tokens) as f32 * trim_ratio) as usize
    } else {
        model_context_size
    };

    tracing::info!("Using context size of {}", size_allowed);

    let mut size_so_far = 0;

    // Add a system message if we have one
    if let Some(system_prompt) = &system_prompt {
        size_so_far = add_message(
            &mut messages,
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(system_prompt.clone()),
                tool_call_id: None,
                tool_calls: None,
                name: None,
            },
            size_so_far,
            size_allowed,
        );
    }

    let mut related_context: Vec<&RelatedContext> = related_context.iter().rev().collect();
    let mut context_so_far: String = Default::default();

    // We need to reverse the history so that adding it to the messages
    // Puts them in the correct order
    let mut history: Vec<ChatCompletionMessage> = history.into_iter().rev().collect();

    // Keep adding history and context until meet the requirements of the prompt
    while size_so_far < size_allowed {
        // Add some relevant context
        if let Some(rel_context) = related_context.pop() {
            let size_rel_context = token_count_from_string(&rel_context.chunk_text) as usize;

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

        // Expand all the chats we have into the corresponding Messages
        if let Some(hist) = history.pop() {
            size_so_far += add_message(&mut messages, hist, size_so_far, size_allowed);
        }

        if history.is_empty() && related_context.is_empty() {
            break;
        }
    }

    tracing::debug!("{:?}", &messages);

    (messages, chunk_ids)
}

// Only add a message if the context doesn't overflow
fn add_message(
    messages: &mut Vec<ChatCompletionMessage>,
    message_to_add: ChatCompletionMessage,
    size_so_far: usize,
    size_allowed: usize,
) -> usize {
    let size: usize = token_count(vec![message_to_add.clone()]) as usize;

    if (size + size_so_far) < size_allowed {
        messages.push(message_to_add);
        return size_so_far + size;
    }

    size_so_far
}
