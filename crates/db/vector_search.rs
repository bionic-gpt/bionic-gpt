use crate::TokioPostgresError;
use crate::Transaction;

pub struct RelatedContext {
    pub chunk_id: i32,
    pub chunk_text: String,
}

// Query the vector database using a similarity search.
pub async fn get_related_context(
    transaction: &Transaction<'_>,
    dataset_ids: &[i32],
    limit: i32,
    embeddings: Vec<f32>,
) -> Result<Vec<RelatedContext>, TokioPostgresError> {
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
                        rag.chunks
                    WHERE
                        document_id IN (
                            SELECT id FROM rag.documents WHERE dataset_id = ANY($1)
                        )
                    ORDER BY 
                        embeddings <-> $2 
                    LIMIT $3;
                    ",
            &[&dataset_ids, &embedding_data, &(limit as i64)],
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
