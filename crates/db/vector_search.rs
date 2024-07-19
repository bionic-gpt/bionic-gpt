use crate::queries::prompts;
use crate::TokioPostgresError;
use crate::Transaction;

pub struct RelatedContext {
    pub chunk_id: i32,
    pub chunk_text: String,
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
pub async fn get_related_context(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    team_id: i32,
    limit: i32,
    embeddings: Vec<f32>,
) -> Result<Vec<RelatedContext>, TokioPostgresError> {
    // Which datasets does the prompt use
    let datasets = prompts::prompt_datasets()
        .bind(transaction, &prompt_id)
        .all()
        .await?;
    // We just need the id's
    let datasets: Vec<i32> = datasets.iter().map(|dataset| dataset.dataset_id).collect();

    // Format the embeddings in PGVector format
    let embedding_data = pgvector::Vector::from(embeddings.clone());

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

#[derive(Clone, PartialEq, Debug)]
pub struct HistoryResult {
    pub conversation_id: i64,
    pub summary: String,
    pub created_at: String,
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
pub async fn search_history(
    transaction: &Transaction<'_>,
    user_id: i32,
    limit: i32,
    embeddings: Vec<f32>,
) -> Result<Vec<HistoryResult>, TokioPostgresError> {
    // Format the embeddings in PGVector format
    let embedding_data = pgvector::Vector::from(embeddings.clone());

    // Find sections of documents that are related to the users question
    let responses = transaction
        .query(
            "
                SELECT 
                    conv.id::bigint,
                    c.response,
                    c.created_at::Text 
                FROM 
                    chats c
                LEFT JOIN
                    conversations conv
                ON conv.id = c.conversation_id
                WHERE
                    conv.user_id = $1
                ORDER BY 
                    request_embeddings <-> $2 
                LIMIT $3;
            ",
            &[&user_id, &embedding_data, &(limit as i64)],
        )
        .await?;

    // Just get the text from the returned rows
    let results: Vec<HistoryResult> = responses
        .into_iter()
        .map(|content| HistoryResult {
            conversation_id: content.get(0),
            summary: content.get(1),
            created_at: content.get(2),
        })
        .collect();

    Ok(results)
}
