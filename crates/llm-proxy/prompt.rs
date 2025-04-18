use crate::errors::CustomError;
use db::queries::{chats, chats_chunks, prompts};
use db::{Chat, RelatedContext, Transaction};
use openai_api::{Message, ToolCall, ToolCallFunction};
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

    Ok(messages)
}

pub async fn generate_prompt(
    model_context_size: usize,
    max_tokens: usize,
    trim_ratio: f32,
    system_prompt: Option<String>,
    mut history: Vec<Chat>,
    question: Vec<Message>,
    related_context: Vec<RelatedContext>,
) -> (Vec<Message>, Vec<i32>) {
    let mut messages: Vec<Message> = Default::default();
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
            Message {
                role: "system".to_string(),
                content: system_prompt.clone(),
                tool_call_id: None,
                tool_calls: None,
                name: None,
            },
            size_so_far,
            size_allowed,
        );
    }

    // Add the messages that have come from the UI or the API
    // This may already overflow the context!!
    for message in question.into_iter() {
        size_so_far = add_message(&mut messages, message, size_so_far, size_allowed);
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
                    messages[0].content = replaced;
                }
                size_so_far += size_rel_context;
            }
        }

        // Expand all the chats we have into the corresponding Messages
        if let Some(hist) = history.pop() {
            // Add the history in before the last message
            if let Some(top_message) = messages.pop() {
                let chat_messages = convert_chat_to_messages(hist);
                for message_to_add in chat_messages {
                    size_so_far +=
                        add_message(&mut messages, message_to_add, size_so_far, size_allowed);
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

pub fn convert_chat_to_messages(chat: Chat) -> Vec<Message> {
    let mut messages: Vec<Message> = Default::default();
    if let Some(function_call) = chat.function_call {
        // Parse the function call JSON to extract necessary information
        if let Ok(function_call_json) = serde_json::from_str::<serde_json::Value>(&function_call) {
            let id = function_call_json["id"]
                .as_str()
                .unwrap_or("call_id")
                .to_string();
            let function_type = function_call_json["type"]
                .as_str()
                .unwrap_or("function")
                .to_string();
            let function_name = function_call_json["function"]["name"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let function_arguments = function_call_json["function"]["arguments"].to_string();

            // Create an assistant message with tool_calls
            messages.push(Message {
                role: "assistant".to_string(),
                content: "".to_string(),
                tool_call_id: None,
                tool_calls: Some(vec![ToolCall {
                    id: id.clone(),
                    r#type: function_type,
                    function: ToolCallFunction {
                        name: function_name.clone(),
                        arguments: function_arguments,
                    },
                }]),
                name: None,
            });

            // Add tool response if results exist
            if let Some(results) = chat.function_call_results {
                messages.push(Message {
                    role: "tool".to_string(),
                    content: results,
                    tool_call_id: Some(id),
                    name: Some(function_name),
                    tool_calls: None,
                });
            }
        } else {
            // Fallback if JSON parsing fails
            messages.push(Message {
                role: "function".to_string(),
                content: function_call,
                tool_call_id: None,
                tool_calls: None,
                name: None,
            });

            if let Some(results) = chat.function_call_results {
                messages.push(Message {
                    role: "tool".to_string(),
                    content: results,
                    tool_call_id: None,
                    tool_calls: None,
                    name: None,
                });
            }
        }
    } else {
        messages.push(Message {
            role: "user".to_string(),
            content: chat.user_request,
            tool_call_id: None,
            tool_calls: None,
            name: None,
        });
        if let Some(response) = chat.response {
            messages.push(Message {
                role: "assistant".to_string(),
                content: response,
                tool_call_id: None,
                tool_calls: None,
                name: None,
            });
        }
    };
    messages
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
    messages: &mut Vec<Message>,
    message_to_add: Message,
    size_so_far: usize,
    size_allowed: usize,
) -> usize {
    let request = ChatCompletionRequestMessage {
        role: message_to_add.role.clone(),
        content: Some(message_to_add.content.clone()),
        name: None,
        function_call: None,
    };

    let size = num_tokens_from_messages("gpt-4", &[request.clone()]).unwrap();

    if (size + size_so_far) < size_allowed {
        messages.push(message_to_add);
        return size_so_far + size;
    }

    size_so_far
}
