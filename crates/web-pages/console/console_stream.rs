#![allow(non_snake_case)]

use crate::routes;
use db::{authz::Rbac, ChatRole};
use dioxus::prelude::*;
use std::collections::HashMap;
use tool_runtime::{parse_reasoning, parse_tool_calls, ToolCall};

use super::assistant_activity::{ActivityAction, AssistantActivity};
use super::assistant_response::AssistantResponse;
use super::{ChatWithChunks, PendingChatState, CONSOLE_CONTENT_WIDTH};

#[derive(Clone, Debug, Default, PartialEq)]
struct AssistantMessageData {
    reasoning: Vec<String>,
    actions: Vec<ActivityAction>,
    responses: Vec<String>,
    streaming: Option<(i64, String)>,
}

impl AssistantMessageData {
    fn is_empty(&self) -> bool {
        self.reasoning.is_empty()
            && self.actions.is_empty()
            && self.responses.is_empty()
            && self.streaming.is_none()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ConversationTurn {
    user_message: Option<String>,
    assistant: AssistantMessageData,
}

fn visible_reasoning(tool_calls: Option<&str>) -> Vec<String> {
    parse_reasoning(tool_calls)
        .into_iter()
        .map(|reasoning| reasoning.display_text())
        .filter(|text| !text.trim().is_empty())
        .collect()
}

fn current_turn(turns: &mut Vec<ConversationTurn>) -> &mut ConversationTurn {
    if turns.is_empty() {
        turns.push(ConversationTurn::default());
    }
    turns.last_mut().expect("a conversation turn was inserted")
}

fn build_conversation_turns(
    chat_history: &[ChatWithChunks],
    pending_chat_state: &PendingChatState,
    team_id: &str,
) -> Vec<ConversationTurn> {
    let tool_call_index = build_tool_call_index(chat_history);
    let mut turns = Vec::new();

    // History reaches this component newest-first. Build canonical turns first,
    // then reverse only the outer list for the bottom-anchored flex container.
    for chat_with_chunks in chat_history.iter().rev() {
        let chat = &chat_with_chunks.chat;
        match chat.role {
            ChatRole::Assistant => {
                let turn = current_turn(&mut turns);
                turn.assistant
                    .reasoning
                    .extend(visible_reasoning(chat.tool_calls.as_deref()));
                if let Some(content) = chat.content.as_ref().filter(|content| !content.is_empty()) {
                    turn.assistant.responses.push(content.clone());
                }
            }
            ChatRole::Tool => {
                let tool_call = chat
                    .tool_call_id
                    .as_ref()
                    .and_then(|id| tool_call_index.get(id))
                    .cloned();
                current_turn(&mut turns)
                    .assistant
                    .actions
                    .push(ActivityAction {
                        chat_id: chat.id as i64,
                        pending: false,
                        tool_call_id: chat.tool_call_id.clone(),
                        tool_call,
                        response: chat.content.clone(),
                    });
            }
            _ => turns.push(ConversationTurn {
                user_message: Some(chat.content.clone().unwrap_or_default()),
                assistant: AssistantMessageData::default(),
            }),
        }
    }

    match pending_chat_state {
        PendingChatState::PendingUserChat(pending_chat) => {
            turns.push(ConversationTurn {
                user_message: Some(pending_chat.chat.content.clone().unwrap_or_default()),
                assistant: AssistantMessageData {
                    streaming: Some((pending_chat.chat.id as i64, team_id.to_string())),
                    ..AssistantMessageData::default()
                },
            });
        }
        PendingChatState::PendingToolChats(tool_chats, last_chat_id) => {
            let turn = current_turn(&mut turns);
            for chat in tool_chats {
                let tool_call = chat
                    .tool_call_id
                    .as_ref()
                    .and_then(|id| tool_call_index.get(id))
                    .cloned();
                turn.assistant.actions.push(ActivityAction {
                    chat_id: chat.id as i64,
                    pending: true,
                    tool_call_id: chat.tool_call_id.clone(),
                    tool_call,
                    response: chat.content.clone(),
                });
            }
            turn.assistant.streaming = Some((*last_chat_id as i64, team_id.to_string()));
        }
        PendingChatState::None => {}
    }

    turns.reverse();
    turns
}

#[component]
pub fn ConsoleStream(
    team_id: String,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    is_tts_disabled: bool,
    rbac: Rbac,
) -> Element {
    let conversation_turns = build_conversation_turns(&chat_history, &pending_chat_state, &team_id);

    rsx! {
        div {
            class: "flex-1 min-h-0",
            if rbac.can_view_system_prompt() {
                for chat_with_chunks in chat_history.iter() {
                    super::prompt_modal::PromptModal {
                        trigger_id: format!("show-prompt-{}", chat_with_chunks.chat.id),
                        prompt: "{}".to_string(),
                        chunks: chat_with_chunks.chunks.clone(),
                        rbac: rbac.clone()
                    }
                }
            }
            div {
                class: "h-full flex flex-col-reverse overflow-y-auto",
                for turn in conversation_turns {
                    div {
                        class: "conversation-turn {CONSOLE_CONTENT_WIDTH} flex flex-col gap-5 py-5 sm:py-7",
                        if let Some(user_message) = turn.user_message {
                            UserMessage { user_message }
                        }
                        if !turn.assistant.is_empty() {
                            AssistantMessage {
                                team_id: team_id.clone(),
                                assistant: turn.assistant,
                                is_tts_disabled
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UserMessage(user_message: String) -> Element {
    rsx! {
        div {
            class: "ml-auto max-w-[85%] sm:max-w-2xl rounded-2xl rounded-br-md bg-base-200 px-4 py-3 text-sm sm:text-base whitespace-pre-wrap break-words",
            "{user_message}"
        }
    }
}

#[component]
fn AssistantMessage(
    team_id: String,
    assistant: AssistantMessageData,
    is_tts_disabled: bool,
) -> Element {
    rsx! {
        div {
            class: "assistant-message min-w-0 w-full space-y-4",
            AssistantActivity {
                team_id,
                reasoning: assistant.reasoning,
                actions: assistant.actions
            }
            for response in assistant.responses {
                AssistantResponse {
                    response,
                    is_tts_disabled
                }
            }
            if let Some((chat_id, team_id)) = assistant.streaming {
                StreamingAssistantResponse {
                    chat_id,
                    team_id
                }
            }
        }
    }
}

#[component]
fn StreamingAssistantResponse(chat_id: i64, team_id: String) -> Element {
    rsx! {
        div {
            class: "assistant-response min-w-0 w-full",
            div {
                id: "streaming-chat",
                class: "prose prose-sm sm:prose-base max-w-none whitespace-pre-wrap break-words",
                "data-chatid": "{chat_id}",
                "aria-live": "polite",
                "aria-busy": "true",
                span {
                    class: "text-sm text-base-content/55",
                    "Working…"
                }
            }
            ProcessingForm {
                chat_id,
                team_id
            }
        }
    }
}

#[component]
fn ProcessingForm(chat_id: i64, team_id: String) -> Element {
    rsx! {
        form {
            method: "post",
            id: "chat-form-{chat_id}",
            action: routes::console::UpdateResponse { team_id }.to_string(),
            input {
                name: "chat_id",
                value: "{chat_id}",
                "type": "hidden"
            }
        }
    }
}

fn build_tool_call_index(chat_history: &[ChatWithChunks]) -> HashMap<String, ToolCall> {
    let mut index = HashMap::new();
    for chat_with_chunks in chat_history {
        if chat_with_chunks.chat.role != ChatRole::Assistant {
            continue;
        }

        if let Some(tool_calls_json) = &chat_with_chunks.chat.tool_calls {
            for tool_call in parse_tool_calls(Some(tool_calls_json)) {
                index.insert(tool_call.id.clone(), tool_call.clone());
                if let Some(call_id) = tool_call.call_id.clone() {
                    index.insert(call_id, tool_call);
                }
            }
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use db::{Chat, ChatStatus};
    use serde_json::json;
    use tool_runtime::ToolCallFunction;

    fn epoch() -> DateTime<chrono::FixedOffset> {
        DateTime::UNIX_EPOCH.fixed_offset()
    }

    fn chat(id: i32, role: ChatRole, content: Option<&str>) -> ChatWithChunks {
        ChatWithChunks {
            chat: Chat {
                id,
                conversation_id: 1,
                content: content.map(str::to_string),
                role,
                tool_call_id: None,
                tool_calls: None,
                model_id: 1,
                model_name: "test-model".to_string(),
                attachments: None,
                status: ChatStatus::Success,
                created_at: epoch(),
                updated_at: epoch(),
            },
            chunks: Vec::new(),
        }
    }

    fn chat_with_tool_calls(tool_calls: Vec<ToolCall>) -> ChatWithChunks {
        let mut chat = chat(1, ChatRole::Assistant, None);
        chat.chat.tool_calls = serde_json::to_string(&tool_calls).ok();
        chat
    }

    #[test]
    fn build_tool_call_index_indexes_id_and_call_id() {
        let tool_call = ToolCall {
            id: "provider_item_id".to_string(),
            call_id: Some("provider_call_id".to_string()),
            signature: None,
            additional_params: None,
            function: ToolCallFunction {
                name: "run_bash".to_string(),
                arguments: json!({"query": "bitcoin price"}),
            },
        };
        let history = vec![chat_with_tool_calls(vec![tool_call])];

        let index = build_tool_call_index(&history);

        assert_eq!(
            index["provider_item_id"].function.arguments,
            json!({"query": "bitcoin price"})
        );
        assert_eq!(
            index["provider_call_id"].function.arguments,
            json!({"query": "bitcoin price"})
        );
    }

    #[test]
    fn groups_assistant_activity_and_response_under_the_user_turn() {
        let user = chat(1, ChatRole::User, Some("Find the record"));
        let mut assistant_call = chat(2, ChatRole::Assistant, None);
        assistant_call.chat.tool_calls = serde_json::to_string(&vec![ToolCall {
            id: "call-1".to_string(),
            call_id: None,
            signature: None,
            additional_params: None,
            function: ToolCallFunction {
                name: "run_bash".to_string(),
                arguments: json!({"commands": "sqlite3 customers.db '.tables'"}),
            },
        }])
        .ok();
        let mut tool = chat(3, ChatRole::Tool, Some(r#"{"stdout":"customers"}"#));
        tool.chat.tool_call_id = Some("call-1".to_string());
        let response = chat(4, ChatRole::Assistant, Some("I found the record."));
        let history = vec![response, tool, assistant_call, user];

        let turns = build_conversation_turns(&history, &PendingChatState::None, "team-1");

        assert_eq!(turns.len(), 1);
        assert_eq!(turns[0].user_message.as_deref(), Some("Find the record"));
        assert_eq!(turns[0].assistant.actions.len(), 1);
        assert_eq!(
            turns[0].assistant.responses,
            vec!["I found the record.".to_string()]
        );
    }

    #[test]
    fn pending_user_chat_creates_a_streaming_turn() {
        let pending = chat(7, ChatRole::User, Some("Continue"));
        let pending_state =
            PendingChatState::PendingUserChat(Box::new(super::super::PendingChat {
                chat: pending.chat,
                tool_calls: None,
            }));

        let turns = build_conversation_turns(&[], &pending_state, "team-1");

        assert_eq!(turns.len(), 1);
        assert_eq!(turns[0].user_message.as_deref(), Some("Continue"));
        assert_eq!(
            turns[0].assistant.streaming,
            Some((7, "team-1".to_string()))
        );
    }

    #[test]
    fn pending_tools_join_the_current_turn_as_one_activity_group() {
        let user = chat(1, ChatRole::User, Some("Inspect the data"));
        let mut assistant_call = chat(2, ChatRole::Assistant, None);
        assistant_call.chat.tool_calls = serde_json::to_string(&vec![ToolCall {
            id: "call-1".to_string(),
            call_id: None,
            signature: None,
            additional_params: None,
            function: ToolCallFunction {
                name: "run_bash".to_string(),
                arguments: json!({"commands": "ls"}),
            },
        }])
        .ok();
        let mut pending_tool = chat(3, ChatRole::Tool, None).chat;
        pending_tool.status = ChatStatus::Pending;
        pending_tool.tool_call_id = Some("call-1".to_string());
        let history = vec![assistant_call, user];
        let pending_state = PendingChatState::PendingToolChats(vec![pending_tool], 3);

        let turns = build_conversation_turns(&history, &pending_state, "team-1");

        assert_eq!(turns.len(), 1);
        assert_eq!(turns[0].assistant.actions.len(), 1);
        assert!(turns[0].assistant.actions[0].pending);
        assert_eq!(
            turns[0].assistant.actions[0]
                .tool_call
                .as_ref()
                .map(|call| call.function.name.as_str()),
            Some("run_bash")
        );
        assert_eq!(
            turns[0].assistant.streaming,
            Some((3, "team-1".to_string()))
        );
    }
}
