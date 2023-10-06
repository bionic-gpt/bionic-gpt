use crate::api_reverse_proxy::Message;
use crate::errors::CustomError;
use db::queries::prompts;
use db::{DatasetConnection, Transaction};
use tiktoken_rs::ChatCompletionRequestMessage;

pub async fn execute_prompt(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    organisation_id: i32,
    question: &str,
) -> Result<Vec<Message>, CustomError> {
    // Get the prompt
    let prompt = prompts::prompt()
        .bind(transaction, &prompt_id, &organisation_id)
        .one()
        .await?;

    // Get related context
    let related_context = get_related_context(
        transaction,
        question,
        prompt.dataset_connection,
        prompt_id,
        organisation_id,
    )
    .await?;

    let messages = generate_prompt(
        prompt.model_context_size as usize,
        prompt.max_tokens as usize,
        prompt.template,
        Default::default(),
        question,
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
    template: String,
    _history: Vec<String>,
    question: &str,
    related_context: Vec<String>,
) -> Vec<ChatCompletionRequestMessage> {
    // This is the space we have to fill
    let _context_size = model_context_size - max_tokens;

    let system_prompt = if related_context.is_empty() {
        template
    } else {
        format!("{}\n\nContext information is below.\n--------------------\n{{context_str}}\n--------------------", template)
    };

    let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestMessage {
            role: "system".to_string(),
            content: Some(system_prompt),
            name: None,
            function_call: None,
        },
        ChatCompletionRequestMessage {
            role: "user".to_string(),
            content: Some(question.to_string()),
            name: None,
            function_call: None,
        },
    ];
    // Keep adding history and context until meet the requirements of the prompt
    //while size_so_far < context_size {
    //    size_so_far += num_tokens_from_messages("gpt-4", &messages).unwrap();
    //}

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
                                embeddings <-> $2 LIMIT 1;
                        ",
                    &[&organisation_id, &embedding_data],
                )
                .await?;

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
                            embeddings <-> $3 LIMIT 1;
                        ",
                    &[&organisation_id, &datasets, &embedding_data],
                )
                .await?;

            // Just get the text from the returned rows
            let related_context: Vec<String> = related_context
                .into_iter()
                .map(|content| content.get(0))
                .collect();
            Ok(related_context)
        }
    }
}
