use crate::errors::CustomError;
use db::queries::{prompts, runtime_settings};
use db::Transaction;
use db::{Chat, ChatRole};
use rig::message::{AssistantContent, Message};
use rig::OneOrMany;
use tool_runtime::{parse_reasoning, parse_tool_calls, ToolCall};

/// Converts database chats into rig-native messages.
pub fn convert_chat_to_messages(conversation: Vec<Chat>) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();

    for chat in conversation {
        let tool_calls: Vec<ToolCall> = parse_tool_calls(chat.tool_calls.as_deref());

        let content = chat.content.unwrap_or_default();

        let message = match chat.role {
            ChatRole::Assistant => {
                let mut items: Vec<AssistantContent> = Vec::new();
                for reasoning in parse_reasoning(chat.tool_calls.as_deref()) {
                    items.push(AssistantContent::Reasoning(reasoning));
                }

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
            ChatRole::Tool => {
                let tool_call_id = chat.tool_call_id.unwrap_or_else(|| "tool_call".to_string());
                Message::tool_result_with_call_id(tool_call_id.clone(), Some(tool_call_id), content)
            }
            ChatRole::System | ChatRole::Developer => Message::system(content),
            ChatRole::User => Message::user(content),
        };

        messages.push(message);
    }

    messages
}

pub async fn execute_prompt(
    transaction: &Transaction<'_>,
    prompt: prompts::SinglePrompt,
    _conversation_id: Option<i64>,
    include_skills: bool,
    chat_history: Vec<Message>,
) -> Result<Vec<Message>, CustomError> {
    tracing::info!("Retrieved {} history items", chat_history.len());

    let trim_ratio = (prompt.trim_ratio as f32) / 100.0;
    let max_completion_tokens = prompt.max_completion_tokens.unwrap_or(0) as usize;
    let runtime_system_prompt = runtime_settings::default_system_prompt()
        .bind(transaction)
        .one()
        .await?
        .value;
    let tool_context = if include_skills {
        let skill_summaries = db::queries::skills::visible_skill_summaries()
            .bind(transaction)
            .all()
            .await?;
        tool_runtime::skills::available_skills_prompt_section_with_custom(skill_summaries)
    } else {
        None
    };

    Ok(generate_prompt(
        prompt.model_context_size as usize,
        max_completion_tokens,
        trim_ratio,
        Some(runtime_system_prompt),
        prompt.system_prompt,
        tool_context,
        chat_history,
    )
    .await)
}

pub async fn generate_prompt(
    model_context_size: usize,
    max_completion_tokens: usize,
    trim_ratio: f32,
    runtime_system_prompt: Option<String>,
    system_prompt: Option<String>,
    runtime_context: Option<String>,
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

    let system_prompt =
        combine_system_prompt(runtime_system_prompt, system_prompt, runtime_context);

    if let Some(system_prompt) = &system_prompt {
        size_so_far = add_message(
            &mut messages,
            Message::system(system_prompt.clone()),
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

fn combine_system_prompt(
    runtime_system_prompt: Option<String>,
    system_prompt: Option<String>,
    runtime_context: Option<String>,
) -> Option<String> {
    let prompt =
        combine_optional_sections(vec![runtime_system_prompt, system_prompt, runtime_context]);
    if prompt.is_empty() {
        None
    } else {
        Some(prompt)
    }
}

fn combine_optional_sections(sections: Vec<Option<String>>) -> String {
    sections
        .into_iter()
        .flatten()
        .filter(|section| !section.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn add_message(
    messages: &mut Vec<Message>,
    message_to_add: Message,
    size_so_far: usize,
    size_allowed: usize,
) -> usize {
    let size = estimate_message_tokens(&message_to_add);

    if (size + size_so_far) < size_allowed {
        messages.push(message_to_add);
        return size_so_far + size;
    }

    size_so_far
}

pub(crate) fn estimate_message_tokens(message: &Message) -> usize {
    let bytes = serde_json::to_vec(message).unwrap_or_default().len();
    (bytes / 4).max(1)
}
