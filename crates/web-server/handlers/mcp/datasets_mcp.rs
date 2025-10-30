use super::{
    default_arguments, json_response, negotiate_protocol_version, should_validate_api_key,
    validate_api_key, InitializeParams, JsonRpcRequest, JsonRpcResponse, McpTool, ToolCallParams,
    DEFAULT_PROTOCOL_VERSION, JSONRPC_VERSION, SERVER_NAME, SERVER_VERSION,
};
use crate::CustomError;
use axum::{
    extract::Extension,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json, Router,
};
use axum_extra::routing::{RouterExt, TypedPath};
use db::Pool;
use pgvector::Vector;
use serde::Deserialize;
use serde_json::{json, Value};
use std::cmp;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/mcp/datasets/{dataset_id}")]
pub struct DatasetJsonRpcPath {
    pub dataset_id: Uuid,
}

pub(super) fn routes() -> Router {
    Router::new().typed_post(handle_dataset_json_rpc)
}

#[derive(Debug)]
enum DatasetResolveError {
    NotFound(Uuid),
    Database(String),
}

impl From<db::PoolError> for DatasetResolveError {
    fn from(err: db::PoolError) -> Self {
        DatasetResolveError::Database(err.to_string())
    }
}

impl From<db::TokioPostgresError> for DatasetResolveError {
    fn from(err: db::TokioPostgresError) -> Self {
        DatasetResolveError::Database(err.to_string())
    }
}

struct DatasetContext {
    dataset_id: i32,
    external_id: Uuid,
    team_id: i32,
    name: String,
    embeddings_model_id: i32,
}

pub async fn handle_dataset_json_rpc(
    DatasetJsonRpcPath { dataset_id }: DatasetJsonRpcPath,
    Extension(pool): Extension<Pool>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Response, CustomError> {
    let enforce_api_key = should_validate_api_key();

    let api_key_team_id = if enforce_api_key {
        let authorization_header = headers
            .get(AUTHORIZATION)
            .ok_or_else(|| CustomError::Authentication("You need an API key".to_string()))?;
        let authorization_value = authorization_header
            .to_str()
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;
        let api_key_value = authorization_value
            .strip_prefix("Bearer ")
            .unwrap_or(authorization_value)
            .trim();

        if api_key_value.is_empty() {
            return Err(CustomError::Authentication("Invalid API Key".to_string()));
        }

        Some(validate_api_key(&pool, api_key_value).await?)
    } else {
        None
    };

    let id_from_payload = payload.get("id").cloned().unwrap_or(Value::Null);
    let request: JsonRpcRequest = match serde_json::from_value(payload.clone()) {
        Ok(req) => req,
        Err(err) => {
            let response = JsonRpcResponse::failure(
                id_from_payload,
                -32600,
                "Invalid JSON-RPC request".to_string(),
                Some(json!({ "details": err.to_string() })),
            );
            return Ok(json_response(response));
        }
    };

    let request_id = request.id.clone().unwrap_or(Value::Null);

    if request.jsonrpc != JSONRPC_VERSION {
        let response = JsonRpcResponse::failure(
            request_id.clone(),
            -32600,
            format!("Unsupported JSON-RPC version: {}", request.jsonrpc),
            None,
        );
        return Ok(json_response(response));
    }

    let dataset_context = match resolve_dataset_context(&pool, dataset_id).await {
        Ok(ctx) => ctx,
        Err(err) => {
            let (code, message, data) = match err {
                DatasetResolveError::NotFound(id) => (
                    -32004,
                    "Unknown dataset".to_string(),
                    Some(json!({ "datasetId": id })),
                ),
                DatasetResolveError::Database(details) => (
                    -32603,
                    "Database error while loading dataset".to_string(),
                    Some(json!({ "details": details })),
                ),
            };

            let response = JsonRpcResponse::failure(request_id.clone(), code, message, data);
            return Ok(json_response(response));
        }
    };

    if let Some(team_id) = api_key_team_id {
        if dataset_context.team_id != team_id {
            return Err(CustomError::Authentication(
                "API key is not authorized for this dataset".to_string(),
            ));
        }
    }

    match request.method.as_str() {
        "initialize" => {
            let params: InitializeParams = match serde_json::from_value(request.params.clone()) {
                Ok(p) => p,
                Err(err) => {
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32602,
                        "Invalid parameters for initialize".to_string(),
                        Some(json!({ "details": err.to_string() })),
                    );
                    return Ok(json_response(response));
                }
            };

            let negotiated_protocol =
                match negotiate_protocol_version(params.protocol_version.as_deref()) {
                    Ok(version) => version,
                    Err(message) => {
                        let response =
                            JsonRpcResponse::failure(request_id.clone(), -32602, message, None);
                        return Ok(json_response(response));
                    }
                };

            let response = JsonRpcResponse::success(
                request_id.clone(),
                json!({
                    "protocolVersion": negotiated_protocol,
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "serverInfo": {
                        "name": SERVER_NAME,
                        "version": SERVER_VERSION,
                    },
                    "metadata": dataset_metadata(&dataset_context)
                }),
            );
            Ok(json_response(response))
        }
        "session.initialize" => {
            let response = JsonRpcResponse::success(
                request_id.clone(),
                json!({
                    "protocolVersion": DEFAULT_PROTOCOL_VERSION,
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "metadata": dataset_metadata(&dataset_context)
                }),
            );
            Ok(json_response(response))
        }
        "notifications/initialized" => {
            if request.id.is_some() {
                let response = JsonRpcResponse::failure(
                    request_id.clone(),
                    -32600,
                    "notifications/initialized must not include an id".to_string(),
                    None,
                );
                Ok(json_response(response))
            } else {
                Ok(StatusCode::ACCEPTED.into_response())
            }
        }
        "tools/list" => {
            let tools = dataset_tools(&dataset_context);
            let response = JsonRpcResponse::success(request_id.clone(), json!({ "tools": tools }));
            Ok(json_response(response))
        }
        "tools/call" => {
            let params: ToolCallParams = match serde_json::from_value(request.params.clone()) {
                Ok(p) => p,
                Err(err) => {
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32602,
                        "Invalid parameters for tools/call".to_string(),
                        Some(json!({ "details": err.to_string() })),
                    );
                    return Ok(json_response(response));
                }
            };

            let ToolCallParams { name, arguments } = params;

            let result = match name.as_str() {
                "get_documents" => {
                    let args: GetDocumentsParams = match parse_optional_arguments(arguments) {
                        Ok(args) => args,
                        Err(err) => {
                            let response = JsonRpcResponse::failure(
                                request_id.clone(),
                                -32602,
                                "Invalid arguments for get_documents".to_string(),
                                Some(json!({ "details": err })),
                            );
                            return Ok(json_response(response));
                        }
                    };
                    get_documents(&pool, &dataset_context, args).await
                }
                "get_document" => {
                    let args: GetDocumentParams = match parse_required_arguments(arguments) {
                        Ok(args) => args,
                        Err(err) => {
                            let response = JsonRpcResponse::failure(
                                request_id.clone(),
                                -32602,
                                "Invalid arguments for get_document".to_string(),
                                Some(json!({ "details": err })),
                            );
                            return Ok(json_response(response));
                        }
                    };
                    get_document(&pool, &dataset_context, args).await
                }
                "search_context" => {
                    let args: SearchContextParams = match parse_required_arguments(arguments) {
                        Ok(args) => args,
                        Err(err) => {
                            let response = JsonRpcResponse::failure(
                                request_id.clone(),
                                -32602,
                                "Invalid arguments for search_context".to_string(),
                                Some(json!({ "details": err })),
                            );
                            return Ok(json_response(response));
                        }
                    };
                    search_context(&pool, &dataset_context, args).await
                }
                other => Err(DatasetToolError::UnknownTool(other.to_string())),
            };

            match result {
                Ok(result) => {
                    let response = JsonRpcResponse::success(
                        request_id.clone(),
                        json!({
                            "content": [
                                {
                                    "type": "object",
                                    "object": result
                                }
                            ]
                        }),
                    );
                    Ok(json_response(response))
                }
                Err(DatasetToolError::InvalidParams(message)) => {
                    let response =
                        JsonRpcResponse::failure(request_id.clone(), -32602, message, None);
                    Ok(json_response(response))
                }
                Err(DatasetToolError::NotFound(message)) => {
                    let response =
                        JsonRpcResponse::failure(request_id.clone(), -32004, message, None);
                    Ok(json_response(response))
                }
                Err(DatasetToolError::UnknownTool(message)) => {
                    let response =
                        JsonRpcResponse::failure(request_id.clone(), -32601, message, None);
                    Ok(json_response(response))
                }
                Err(DatasetToolError::Internal(message)) => {
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32603,
                        "Dataset tool execution failed".to_string(),
                        Some(json!({ "details": message })),
                    );
                    Ok(json_response(response))
                }
            }
        }
        _ => {
            let response =
                JsonRpcResponse::failure(request_id, -32601, "Unknown method".to_string(), None);
            Ok(json_response(response))
        }
    }
}

fn dataset_metadata(context: &DatasetContext) -> Value {
    json!({
        "datasetId": context.dataset_id,
        "datasetExternalId": context.external_id,
        "datasetName": context.name,
        "datasetSlug": "datasets",
    })
}

fn dataset_tools(context: &DatasetContext) -> Vec<McpTool> {
    let metadata = Some(dataset_metadata(context));

    vec![
        McpTool {
            name: "get_documents".to_string(),
            description: Some(format!(
                "List documents for the dataset \"{}\".",
                context.name
            )),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 100,
                        "description": "Maximum number of documents to return (default 20)"
                    },
                    "offset": {
                        "type": "integer",
                        "minimum": 0,
                        "description": "Number of documents to skip from the start (default 0)"
                    }
                }
            }),
            metadata: metadata.clone(),
        },
        McpTool {
            name: "get_document".to_string(),
            description: Some(format!(
                "Fetch details and chunks for a specific document in the dataset \"{}\".",
                context.name
            )),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "document_id": {
                        "type": "integer",
                        "description": "Numeric identifier of the document"
                    },
                    "include_chunks": {
                        "type": "boolean",
                        "description": "Whether to include document chunks (default true)"
                    },
                    "chunk_limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 500,
                        "description": "Maximum number of chunks to include (default 50)"
                    }
                },
                "required": ["document_id"]
            }),
            metadata: metadata.clone(),
        },
        McpTool {
            name: "search_context".to_string(),
            description: Some(format!(
                "Semantic search across the dataset \"{}\" returning relevant chunks.",
                context.name
            )),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query text"
                    },
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 25,
                        "description": "Maximum number of chunks to return (default 5)"
                    }
                },
                "required": ["query"]
            }),
            metadata,
        },
    ]
}

async fn resolve_dataset_context(
    pool: &Pool,
    dataset_id: Uuid,
) -> Result<DatasetContext, DatasetResolveError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let row = db::queries::datasets::dataset_by_external_id()
        .bind(&transaction, &dataset_id)
        .opt()
        .await?;

    transaction.commit().await?;

    match row {
        Some(row) => Ok(DatasetContext {
            dataset_id: row.id,
            team_id: row.team_id,
            external_id: row.external_id,
            name: row.name,
            embeddings_model_id: row.embeddings_model_id,
        }),
        None => Err(DatasetResolveError::NotFound(dataset_id)),
    }
}

#[derive(Debug)]
enum DatasetToolError {
    InvalidParams(String),
    NotFound(String),
    Internal(String),
    UnknownTool(String),
}

#[derive(Default, Deserialize)]
struct GetDocumentsParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize)]
struct GetDocumentParams {
    document_id: i32,
    #[serde(default)]
    include_chunks: Option<bool>,
    #[serde(default)]
    chunk_limit: Option<i64>,
}

#[derive(Deserialize)]
struct SearchContextParams {
    query: String,
    #[serde(default)]
    limit: Option<i64>,
}

fn parse_optional_arguments<T>(value: Value) -> Result<T, String>
where
    T: Default + for<'de> Deserialize<'de>,
{
    if matches!(value, Value::Null) {
        Ok(T::default())
    } else {
        serde_json::from_value(value).map_err(|err| err.to_string())
    }
}

fn parse_required_arguments<T>(value: Value) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_value(if matches!(value, Value::Null) {
        default_arguments()
    } else {
        value
    })
    .map_err(|err| err.to_string())
}

async fn get_documents(
    pool: &Pool,
    context: &DatasetContext,
    params: GetDocumentsParams,
) -> Result<Value, DatasetToolError> {
    let limit = params.limit.unwrap_or(20);
    if limit < 1 {
        return Err(DatasetToolError::InvalidParams(
            "limit must be greater than 0".to_string(),
        ));
    }
    let limit = cmp::min(limit, 100);

    let offset = params.offset.unwrap_or(0);
    if offset < 0 {
        return Err(DatasetToolError::InvalidParams(
            "offset cannot be negative".to_string(),
        ));
    }

    let mut client = pool
        .get()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    apply_customer_key(&transaction).await?;

    let rows = transaction
        .query(
            "
            SELECT
                d.id,
                d.file_name,
                d.content_size,
                d.created_at,
                d.updated_at,
                d.failure_reason,
                (SELECT COUNT(*) FROM chunks c WHERE c.document_id = d.id) AS chunk_count
            FROM documents d
            WHERE d.dataset_id = $1
            ORDER BY d.updated_at DESC
            LIMIT $2 OFFSET $3
            ",
            &[&context.dataset_id, &limit, &offset],
        )
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    transaction
        .commit()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    let documents: Vec<Value> = rows
        .into_iter()
        .map(|row| {
            let created_at = row.get::<_, time::OffsetDateTime>(3);
            let updated_at = row.get::<_, time::OffsetDateTime>(4);
            json!({
                "id": row.get::<_, i32>(0),
                "file_name": row.get::<_, String>(1),
                "content_size": row.get::<_, i32>(2),
                "created_at": format_timestamp(created_at),
                "updated_at": format_timestamp(updated_at),
                "failure_reason": row.get::<_, Option<String>>(5),
                "chunk_count": row.get::<_, i64>(6),
            })
        })
        .collect();

    Ok(json!({ "documents": documents }))
}

async fn get_document(
    pool: &Pool,
    context: &DatasetContext,
    params: GetDocumentParams,
) -> Result<Value, DatasetToolError> {
    let include_chunks = params.include_chunks.unwrap_or(true);
    let mut client = pool
        .get()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    apply_customer_key(&transaction).await?;

    let document_row = transaction
        .query_opt(
            "
            SELECT
                d.id,
                d.file_name,
                d.content_size,
                d.created_at,
                d.updated_at,
                d.failure_reason
            FROM documents d
            WHERE d.id = $1 AND d.dataset_id = $2
            ",
            &[&params.document_id, &context.dataset_id],
        )
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    let Some(document_row) = document_row else {
        return Err(DatasetToolError::NotFound(format!(
            "Document {} was not found in dataset {}",
            params.document_id, context.dataset_id
        )));
    };

    let created_at = document_row.get::<_, time::OffsetDateTime>(3);
    let updated_at = document_row.get::<_, time::OffsetDateTime>(4);

    let mut document = json!({
        "id": document_row.get::<_, i32>(0),
        "file_name": document_row.get::<_, String>(1),
        "content_size": document_row.get::<_, i32>(2),
        "created_at": format_timestamp(created_at),
        "updated_at": format_timestamp(updated_at),
        "failure_reason": document_row.get::<_, Option<String>>(5),
    });

    if include_chunks {
        let chunk_limit = params.chunk_limit.unwrap_or(50);
        if chunk_limit < 1 {
            return Err(DatasetToolError::InvalidParams(
                "chunk_limit must be greater than 0".to_string(),
            ));
        }
        let chunk_limit = cmp::min(chunk_limit, 500);

        let chunk_rows = transaction
            .query(
                "
                SELECT c.id, c.page_number, c.text
                FROM chunks c
                WHERE c.document_id = $1
                ORDER BY c.page_number ASC, c.id ASC
                LIMIT $2
                ",
                &[&params.document_id, &chunk_limit],
            )
            .await
            .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

        let chunks: Vec<Value> = chunk_rows
            .into_iter()
            .map(|row| {
                json!({
                    "id": row.get::<_, i32>(0),
                    "page_number": row.get::<_, i32>(1),
                    "text": row.get::<_, String>(2),
                })
            })
            .collect();

        document
            .as_object_mut()
            .expect("document json is object")
            .insert("chunks".to_string(), json!(chunks));
    }

    transaction
        .commit()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    Ok(json!({ "document": document }))
}

async fn search_context(
    pool: &Pool,
    context: &DatasetContext,
    params: SearchContextParams,
) -> Result<Value, DatasetToolError> {
    let limit = params.limit.unwrap_or(5);
    if limit < 1 {
        return Err(DatasetToolError::InvalidParams(
            "limit must be greater than 0".to_string(),
        ));
    }
    let limit = cmp::min(limit, 25);

    let mut client = pool
        .get()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    apply_customer_key(&transaction).await?;

    let model = db::queries::models::model()
        .bind(&transaction, &context.embeddings_model_id)
        .one()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    let db::queries::models::Model {
        name: embeddings_model,
        base_url,
        api_key,
        context_size,
        ..
    } = model;

    if base_url.trim().is_empty() {
        return Err(DatasetToolError::Internal(
            "Embedding model missing base URL".to_string(),
        ));
    }

    let embeddings = embeddings_api::get_embeddings(
        &params.query,
        &base_url,
        &embeddings_model,
        context_size,
        &api_key,
    )
    .await
    .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    let embedding_vector = Vector::from(embeddings);

    let chunk_rows = transaction
        .query(
            "
            SELECT
                c.id,
                c.text,
                c.page_number,
                d.id,
                d.file_name,
                (c.embeddings <-> $2) AS distance
            FROM chunks c
            INNER JOIN documents d ON c.document_id = d.id
            WHERE d.dataset_id = $1 AND c.embeddings IS NOT NULL
            ORDER BY c.embeddings <-> $2
            LIMIT $3
            ",
            &[&context.dataset_id, &embedding_vector, &limit],
        )
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    transaction
        .commit()
        .await
        .map_err(|err| DatasetToolError::Internal(err.to_string()))?;

    let chunks: Vec<Value> = chunk_rows
        .into_iter()
        .map(|row| {
            json!({
                "chunk_id": row.get::<_, i32>(0),
                "text": row.get::<_, String>(1),
                "page_number": row.get::<_, i32>(2),
                "document_id": row.get::<_, i32>(3),
                "document_name": row.get::<_, String>(4),
                "distance": row.get::<_, f32>(5),
            })
        })
        .collect();

    Ok(json!({ "chunks": chunks }))
}

async fn apply_customer_key(transaction: &db::Transaction<'_>) -> Result<(), DatasetToolError> {
    if let Some(key) = db::customer_keys::get_customer_key() {
        let escaped = key.replace('\'', "''");
        transaction
            .query(
                &format!("SET LOCAL encryption.root_key = '{}'", escaped),
                &[],
            )
            .await
            .map_err(|err| DatasetToolError::Internal(err.to_string()))?;
    }
    Ok(())
}

fn format_timestamp(value: time::OffsetDateTime) -> String {
    value.format(&Rfc3339).unwrap_or_else(|_| value.to_string())
}
