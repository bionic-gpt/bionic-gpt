use crate::chat_types::{ChatCompletionMessage, ChatCompletionMessageRole};
use crate::errors::CustomError;
use db::queries::{prompt_integrations, prompts};
use db::Transaction;
use tool_runtime::ToolDefinition;
use tool_runtime::{create_tools_from_integrations, get_tools, ToolScope};

// If we are getting called from the API we'll possible have a buch of chat messaages
// that's why chat is a Vec<Message>
// For the UI they'll be just one.
pub async fn execute_prompt(
    _transaction: &Transaction<'_>,
    prompt: prompts::SinglePrompt,
    _conversation_id: Option<i64>,
    chat_history: Vec<ChatCompletionMessage>,
) -> Result<Vec<ChatCompletionMessage>, CustomError> {
    tracing::info!("Retrieved {} history items", chat_history.len());

    let trim_ratio = (prompt.trim_ratio as f32) / 100.0;

    let max_completion_tokens = prompt.max_completion_tokens.unwrap_or(0) as usize;
    let messages = generate_prompt(
        prompt.model_context_size as usize,
        max_completion_tokens,
        trim_ratio,
        prompt.system_prompt,
        chat_history,
    )
    .await;

    Ok(messages)
}

/// Get integration tools for a specific prompt
pub async fn get_prompt_integration_tools(
    transaction: &Transaction<'_>,
    prompt_id: i32,
) -> Result<Vec<ToolDefinition>, CustomError> {
    // Get integrations for this specific prompt using existing transaction
    let prompt_integrations = prompt_integrations::get_prompt_integrations_with_connections()
        .bind(transaction, &prompt_id)
        .all()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get integrations for prompt {}: {}", prompt_id, e);
            CustomError::Database(e.to_string(), std::backtrace::Backtrace::capture())
        })?;

    // Create tools from the integrations
    let external_tools = create_tools_from_integrations(prompt_integrations, None, None).await;

    let mut filtered_tools: Vec<ToolDefinition> = external_tools
        .into_iter()
        .map(|tool| tool.get_tool())
        .collect();

    let datasets = prompts::prompt_datasets()
        .bind(transaction, &prompt_id)
        .all()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get datasets for prompt {}: {}", prompt_id, e);
            CustomError::Database(e.to_string(), std::backtrace::Backtrace::capture())
        })?;

    if !datasets.is_empty() {
        filtered_tools.extend(get_tools(ToolScope::Rag));
    }

    tracing::info!(
        "Retrieved {} integration tools for prompt {}",
        filtered_tools.len(),
        prompt_id
    );
    Ok(filtered_tools)
}

pub async fn generate_prompt(
    model_context_size: usize,
    max_completion_tokens: usize,
    trim_ratio: f32,
    system_prompt: Option<String>,
    history: Vec<ChatCompletionMessage>,
) -> Vec<ChatCompletionMessage> {
    let mut messages: Vec<ChatCompletionMessage> = Default::default();

    // This is the space we have to fill
    let size_allowed = if max_completion_tokens < model_context_size {
        ((model_context_size - max_completion_tokens) as f32 * trim_ratio) as usize
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

    // Newest history messages are processed first so they fill the budget before older ones
    let mut history = history;
    let mut history_messages: Vec<ChatCompletionMessage> = Vec::new();

    // Keep adding history and context until meet the requirements of the prompt
    while size_so_far < size_allowed {
        // Expand all the chats we have into the corresponding Messages
        if let Some(hist) = history.pop() {
            size_so_far = add_message(&mut history_messages, hist, size_so_far, size_allowed);
        }

        if history.is_empty() {
            break;
        }
    }

    history_messages.reverse();
    messages.extend(history_messages);

    tracing::debug!("{:?}", &messages);

    messages
}

// Only add a message if the context doesn't overflow
fn add_message(
    messages: &mut Vec<ChatCompletionMessage>,
    message_to_add: ChatCompletionMessage,
    size_so_far: usize,
    size_allowed: usize,
) -> usize {
    let size: usize = crate::token_count::token_count(vec![message_to_add.clone()]) as usize;

    if (size + size_so_far) < size_allowed {
        messages.push(message_to_add);
        return size_so_far + size;
    }

    size_so_far
}
