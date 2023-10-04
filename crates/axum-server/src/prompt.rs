use crate::api_reverse_proxy::Message;
use crate::errors::CustomError;
use db::queries::prompts;
use db::{DatasetConnection, Transaction};

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
    let related_context = related_context.join(" ");
    let prompt = prompt.template.replace("{context_str}", &related_context);

    let messages: Vec<Message> = vec![
        Message {
            role: "system".to_string(),
            content: prompt,
        },
        Message {
            role: "user".to_string(),
            content: question.to_string(),
        },
    ];

    Ok(messages)
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
