use crate::api_reverse_proxy::Message;
use crate::errors::CustomError;
use db::queries::{chats, prompts};
use db::{Chat, DatasetConnection, Transaction};
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

pub async fn execute_prompt(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    organisation_id: i32,
    conversation_id: Option<i64>,
    chat: Vec<Message>,
) -> Result<Vec<Message>, CustomError> {
    // Get the prompt
    let prompt = prompts::prompt()
        .bind(transaction, &prompt_id, &organisation_id)
        .one()
        .await?;

    let question = if let Some(q) = chat.last() {
        q.content.clone()
    } else {
        "".to_string()
    };

    tracing::info!(prompt.name);
    // Get related context
    let related_context = get_related_context(
        transaction,
        &question,
        prompt.dataset_connection,
        prompt_id,
        organisation_id,
        prompt.max_chunks,
    )
    .await?;
    tracing::info!("Retrieved {} chunks", related_context.len());

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

    let messages = generate_prompt(
        prompt.model_context_size as usize,
        prompt.max_tokens as usize,
        prompt.system_prompt,
        chat_history,
        chat,
        related_context,
    )
    .await;

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
    system_prompt: Option<String>,
    mut history: Vec<Chat>,
    question: Vec<Message>,
    related_context: Vec<String>,
) -> Vec<ChatCompletionRequestMessage> {
    let mut messages: Vec<ChatCompletionRequestMessage> = Default::default();

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

    if let Some(system_prompt) = &system_prompt {
        messages.push(ChatCompletionRequestMessage {
            role: "user".to_string(),
            content: Some(system_prompt.clone()),
            name: None,
            function_call: None,
        });
    }

    for message in question.into_iter() {
        messages.push(ChatCompletionRequestMessage {
            role: message.role,
            content: Some(message.content),
            name: None,
            function_call: None,
        });
    }

    // This is the space we have to fill
    let context_size = model_context_size - max_tokens;
    let mut size_so_far = num_tokens_from_messages("gpt-4", &messages).unwrap();
    let mut related_context: Vec<&String> = related_context.iter().rev().collect();
    let mut context_so_far: String = Default::default();

    // Keep adding history and context until meet the requirements of the prompt
    while size_so_far < context_size {
        // Add some relevant context
        if let Some(rel_context) = related_context.pop() {
            context_so_far.push_str(rel_context);
            context_so_far += "\n";
            if let Some(prompt) = &system_prompt {
                let replaced = prompt.replace("{context_str}", &context_so_far);
                messages[0].content = Some(replaced);
            }
        }

        size_so_far = num_tokens_from_messages("gpt-4", &messages).unwrap();

        if size_so_far >= context_size {
            break;
        }

        // Add some history
        if let Some(hist) = history.pop() {
            // Add the histor in before the last message
            if let Some(top_message) = messages.pop() {
                messages.push(ChatCompletionRequestMessage {
                    role: "user".to_string(),
                    content: Some(hist.user_request),
                    name: None,
                    function_call: None,
                });
                messages.push(ChatCompletionRequestMessage {
                    role: "assistant".to_string(),
                    content: hist.response,
                    name: None,
                    function_call: None,
                });
                messages.push(top_message);
            }
        }

        size_so_far = num_tokens_from_messages("gpt-4", &messages).unwrap();

        if history.is_empty() && related_context.is_empty() {
            break;
        }
    }

    messages
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
async fn get_related_context(
    transaction: &Transaction<'_>,
    message: &str,
    dataset_connection: DatasetConnection,
    prompt_id: i32,
    organisation_id: i32,
    limit: i32,
) -> Result<Vec<String>, CustomError> {
    if dataset_connection == DatasetConnection::None {
        return Ok(Default::default());
    }

    // Turn the users message into something the vector database can use
    let embeddings = open_api::get_embeddings(message)
        .await
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?;

    // Which datasets does the prompt use
    let datasets = prompts::prompt_datasets()
        .bind(transaction, &prompt_id)
        .all()
        .await?;
    // We just need the id's
    let datasets: Vec<i32> = datasets.iter().map(|dataset| dataset.dataset_id).collect();

    // Format the embeddings in PGVector format
    let embedding_data = pgvector::Vector::from(embeddings.clone());

    match dataset_connection {
        DatasetConnection::None => Ok(Default::default()),
        DatasetConnection::All => {
            tracing::info!("About to call");
            // Find sections of documents that are related to the users question
            let related_context = transaction
                .query(
                    "
                    SELECT 
                        text 
                    FROM 
                        embeddings
                    WHERE
                        document_id IN (
                            SELECT id FROM documents WHERE dataset_id IN (
                                SELECT id FROM datasets WHERE organisation_id IN (
                                    SELECT organisation_id FROM organisation_users 
                                    WHERE user_id = current_app_user()
                                    AND organisation_id = $1
                                )
                            )
                        )
                    ORDER BY 
                        embeddings <-> $2 
                    LIMIT $3;
                    ",
                    &[&organisation_id, &embedding_data, &(limit as i64)],
                )
                .await
                .map_err(|e| {
                    tracing::error!("{}", e.to_string());
                    e
                })?;

            // Just get the text from the returned rows
            let related_context: Vec<String> = related_context
                .into_iter()
                .map(|content| content.get(0))
                .collect();
            Ok(related_context)
        }
        DatasetConnection::Selected => {
            // Find sections of documents that are related to the users question
            let related_context = transaction
                .query(
                    "
                    SELECT 
                        text 
                    FROM 
                        embeddings
                    WHERE
                        document_id IN (
                            SELECT id FROM documents WHERE dataset_id IN (
                                SELECT id FROM datasets WHERE organisation_id IN (
                                    SELECT organisation_id FROM organisation_users 
                                    WHERE user_id = current_app_user()
                                    AND organisation_id = $1
                                )
                                AND dataset_id = ANY($2)
                            )
                        )
                    ORDER BY 
                        embeddings <-> $3 
                    LIMIT $4;
                    ",
                    &[
                        &organisation_id,
                        &datasets,
                        &embedding_data,
                        &(limit as i64),
                    ],
                )
                .await
                .map_err(|e| {
                    tracing::error!("{}", e.to_string());
                    e
                })?;

            // Just get the text from the returned rows
            let related_context: Vec<String> = related_context
                .into_iter()
                .map(|content| content.get(0))
                .collect();
            Ok(related_context)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_generate_prompt() {
        let messages = generate_prompt(
            2048,
            1024,
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
        let messages = generate_prompt(
            2048,
            1024,
            Some("You are a helpful asistant".to_string()),
            vec![create_prompt(
                "What time is it?".to_string(),
                "I don't know".to_string(),
            )],
            vec![Message {
                role: "user".to_string(),
                content: "How are you today?".to_string(),
            }],
            vec!["This might help".to_string()],
        )
        .await;

        assert!(messages.len() == 4);

        dbg!(&messages);

        assert!(messages[0].content == Some("You are a helpful asistant\n\nContext information is below.\n--------------------\nThis might help\n\n--------------------".to_string()));
        assert!(messages[3].content == Some("How are you today?".to_string()));
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
