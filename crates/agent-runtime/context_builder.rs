use crate::errors::CustomError;
use db::queries::{prompt_integrations, prompts};
use db::Transaction;
use db::{Chat, ChatRole};
use rig::message::{AssistantContent, Message};
use rig::OneOrMany;
use tool_runtime::ToolDefinition;
use tool_runtime::{
    create_tools_from_integrations, get_tools, parse_tool_calls, ToolCall, ToolScope,
};

/// Converts database chats into rig-native messages.
pub fn convert_chat_to_messages(conversation: Vec<Chat>) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();

    for chat in conversation {
        let tool_calls: Vec<ToolCall> = parse_tool_calls(chat.tool_calls.as_deref());

        let content = chat.content.unwrap_or_default();

        let message = match chat.role {
            ChatRole::Assistant => {
                let mut items: Vec<AssistantContent> = Vec::new();
                if !content.trim().is_empty() {
                    items.push(AssistantContent::text(content));
                }

                for tool_call in tool_calls {
                    items.push(AssistantContent::ToolCall(tool_call));
                }

                let content = OneOrMany::many(items)
                    .unwrap_or_else(|_| OneOrMany::one(AssistantContent::text("")));
                Message::Assistant { id: None, content }
            }
            ChatRole::Tool => Message::tool_result_with_call_id(
                chat.tool_call_id.unwrap_or_else(|| "tool_call".to_string()),
                None,
                content,
            ),
            ChatRole::User | ChatRole::System | ChatRole::Developer => Message::user(content),
        };

        messages.push(message);
    }

    messages
}

pub async fn execute_prompt(
    _transaction: &Transaction<'_>,
    prompt: prompts::SinglePrompt,
    _conversation_id: Option<i64>,
    chat_history: Vec<Message>,
) -> Result<Vec<Message>, CustomError> {
    tracing::info!("Retrieved {} history items", chat_history.len());

    let trim_ratio = (prompt.trim_ratio as f32) / 100.0;
    let max_completion_tokens = prompt.max_completion_tokens.unwrap_or(0) as usize;

    Ok(generate_prompt(
        prompt.model_context_size as usize,
        max_completion_tokens,
        trim_ratio,
        prompt.system_prompt,
        chat_history,
    )
    .await)
}

pub async fn get_prompt_integration_tools(
    transaction: &Transaction<'_>,
    prompt_id: i32,
) -> Result<Vec<ToolDefinition>, CustomError> {
    let prompt_integrations = prompt_integrations::get_prompt_integrations_with_connections()
        .bind(transaction, &prompt_id)
        .all()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get integrations for prompt {}: {}", prompt_id, e);
            CustomError::Database(e.to_string(), std::backtrace::Backtrace::capture())
        })?;

    let external_tools = create_tools_from_integrations(prompt_integrations, None, None).await;
    let mut filtered_tools: Vec<ToolDefinition> = Vec::new();
    for tool in external_tools {
        filtered_tools.push(tool.definition(String::new()).await);
    }

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
    history: Vec<Message>,
) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();

    let size_allowed = if max_completion_tokens < model_context_size {
        ((model_context_size - max_completion_tokens) as f32 * trim_ratio) as usize
    } else {
        model_context_size
    };

    tracing::info!("Using context size of {}", size_allowed);

    let mut size_so_far = 0;

    if let Some(system_prompt) = &system_prompt {
        size_so_far = add_message(
            &mut messages,
            Message::user(system_prompt.clone()),
            size_so_far,
            size_allowed,
        );
    }

    let mut history = history;
    let mut history_messages: Vec<Message> = Vec::new();

    while size_so_far < size_allowed {
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

fn add_message(
    messages: &mut Vec<Message>,
    message_to_add: Message,
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
