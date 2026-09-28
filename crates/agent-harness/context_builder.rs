use crate::errors::CustomError;
use db::queries::{models, runtime_settings};
use db::Transaction;
use db::{Chat, ChatRole};
use rig::message::{AssistantContent, Message};
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

                let content = if items.is_empty() {
                    vec![AssistantContent::text("")]
                } else {
                    items
                };
                Message::Assistant { id: None, content }
            }
            ChatRole::Tool => {
                let tool_call_id = chat.tool_call_id.unwrap_or_else(|| "tool_call".to_string());
                Message::tool_result(tool_call_id, "run_bash", content)
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
    prompt: models::ModelConfig,
    conversation_id: Option<i64>,
    include_skills: bool,
    integration_context: Option<String>,
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
    let skills_context = if include_skills {
        let skill_summaries = db::queries::skills::visible_skill_summaries()
            .bind(transaction)
            .all()
            .await?;
        tool_runtime::skills::available_skills_prompt_section_with_custom(skill_summaries)
    } else {
        None
    };
    let attachment_context = if let Some(conversation_id) = conversation_id {
        Some(attachment_prompt_section(transaction, conversation_id).await?)
    } else {
        None
    };
    let runtime_context = combine_optional_sections(vec![
        skills_context,
        integration_context,
        attachment_context,
    ]);
    let runtime_context = if runtime_context.is_empty() {
        None
    } else {
        Some(runtime_context)
    };

    Ok(generate_prompt(
        prompt.context_size as usize,
        max_completion_tokens,
        trim_ratio,
        Some(runtime_system_prompt),
        prompt.system_prompt,
        runtime_context,
        chat_history,
    )
    .await)
}

async fn attachment_prompt_section(
    transaction: &db::Transaction<'_>,
    conversation_id: i64,
) -> Result<String, CustomError> {
    let attachments = db::queries::attachments::get_by_conversation()
        .bind(transaction, &conversation_id)
        .all()
        .await?;
    if attachments.is_empty() {
        return Ok(String::new());
    }

    let mut section = String::from("## Current attachments\n\n");
    for attachment in attachments {
        let safe_name = sanitize_attachment_name(&attachment.file_name);
        section.push_str(&format!(
            "- **{}**\n  - type: {}\n  - content: /home/user/attachments/{}/content.md\n  - original: /home/user/attachments/{}/original/{}\n",
            attachment.file_name,
            attachment.mime_type,
            attachment.id,
            attachment.id,
            safe_name
        ));
    }
    Ok(section.trim_end().to_string())
}

fn sanitize_attachment_name(file_name: &str) -> String {
    let leaf = file_name.rsplit(['/', '\\']).next().unwrap_or(file_name);
    let sanitized = leaf
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_' | ' ') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if sanitized.is_empty() || sanitized == "." || sanitized == ".." {
        "attachment".to_string()
    } else {
        sanitized
    }
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

    let mut history = group_tool_exchanges(history);
    let mut history_messages: Vec<Message> = Vec::new();

    while size_so_far < size_allowed {
        if let Some(unit) = history.pop() {
            let unit_size: usize = unit.iter().map(estimate_message_tokens).sum();
            if size_so_far + unit_size < size_allowed {
                size_so_far += unit_size;
                history_messages.extend(unit);
            }
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

fn group_tool_exchanges(history: Vec<Message>) -> Vec<Vec<Message>> {
    let mut groups: Vec<Vec<Message>> = Vec::new();
    for message in history {
        if is_tool_result(&message) {
            if groups
                .last()
                .is_some_and(|group| group.iter().any(assistant_has_tool_call))
            {
                groups.last_mut().unwrap().push(message);
            } else {
                groups.push(vec![message]);
            }
        } else {
            groups.push(vec![message]);
        }
    }
    groups
}

fn is_tool_result(message: &Message) -> bool {
    matches!(message, Message::User { content } if content.iter().any(|item| matches!(item, rig::message::UserContent::ToolResult(_))))
}

fn assistant_has_tool_call(message: &Message) -> bool {
    matches!(message, Message::Assistant { content, .. } if content.iter().any(|item| matches!(item, AssistantContent::ToolCall(_))))
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
