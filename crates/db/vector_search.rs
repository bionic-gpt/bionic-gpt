use crate::queries::prompts;
use crate::TokioPostgresError;
use crate::{DatasetConnection, Transaction};

pub struct RelatedContext {
    pub chunk_id: i32,
    pub chunk_text: String,
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
pub async fn get_related_context(
    transaction: &Transaction<'_>,
    dataset_connection: DatasetConnection,
    prompt_id: i32,
    team_id: i32,
    limit: i32,
    embeddings: Vec<f32>,
) -> Result<Vec<RelatedContext>, TokioPostgresError> {
    if dataset_connection == DatasetConnection::None {
        return Ok(Default::default());
    }

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
                        id,
                        text 
                    FROM 
                        chunks
                    WHERE
                        document_id IN (
                            SELECT id FROM documents WHERE dataset_id IN (
                                SELECT id FROM datasets WHERE team_id IN (
                                    SELECT team_id FROM team_users 
                                    WHERE user_id = current_app_user()
                                    AND team_id = $1
                                )
                            )
                        )
                    ORDER BY 
                        embeddings <-> $2 
                    LIMIT $3;
                    ",
                    &[&team_id, &embedding_data, &(limit as i64)],
                )
                .await?;

            // Just get the text from the returned rows
            let related_context: Vec<RelatedContext> = related_context
                .into_iter()
                .map(|content| RelatedContext {
                    chunk_id: content.get(0),
                    chunk_text: content.get(1),
                })
                .collect();
            Ok(related_context)
        }
        DatasetConnection::Selected => {
            // Find sections of documents that are related to the users question
            let related_context = transaction
                .query(
                    "
                    SELECT 
                        id,
                        text 
                    FROM 
                        chunks
                    WHERE
                        document_id IN (
                            SELECT id FROM documents WHERE dataset_id IN (
                                SELECT id FROM datasets WHERE team_id IN (
                                    SELECT team_id FROM team_users 
                                    WHERE user_id = current_app_user()
                                    AND team_id = $1
                                )
                                AND dataset_id = ANY($2)
                            )
                        )
                    ORDER BY 
                        embeddings <-> $3 
                    LIMIT $4;
                    ",
                    &[&team_id, &datasets, &embedding_data, &(limit as i64)],
                )
                .await?;

            // Just get the text from the returned rows
            let related_context: Vec<RelatedContext> = related_context
                .into_iter()
                .map(|content| RelatedContext {
                    chunk_id: content.get(0),
                    chunk_text: content.get(1),
                })
                .collect();
            Ok(related_context)
        }
    }
}
