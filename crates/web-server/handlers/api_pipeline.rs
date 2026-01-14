use crate::{config::Config, CustomError};
use axum::{
    extract::{DefaultBodyLimit, Extension, Multipart},
    http::header::HeaderMap,
    response::IntoResponse,
    routing::post,
    Router,
};
use db::queries;
use db::Pool;
use http::StatusCode;

pub fn routes(config: &Config) -> Router {
    Router::new()
        .route("/v1/document_upload", post(upload))
        .layer(DefaultBodyLimit::max(config.max_upload_size_mb * 1_000_000))
}

pub async fn upload(
    Extension(pool): Extension<Pool>,
    Extension(storage_config): Extension<object_storage::StorageConfig>,
    headers: HeaderMap,
    mut files: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    if let Some(api_key) = headers.get("Authorization") {
        let api_key = api_key.to_str().unwrap().replace("Bearer ", "");

        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        let pipeline = queries::document_pipelines::find_api_key()
            .bind(&transaction, &api_key)
            .one()
            .await?;

        while let Some(file) = files.next_field().await.unwrap() {
            // name of the file with extention
            let name = file.file_name().unwrap().to_string();
            // file data
            let data = file.bytes().await.unwrap().to_vec();

            tracing::info!("Sending document to unstructured");

            let object_id = object_storage::upload(
                &storage_config,
                pipeline.user_id,
                pipeline.team_id,
                &name,
                &data,
            )
            .await?;

            let _document_id = queries::documents::insert_with_object()
                .bind(
                    &transaction,
                    &pipeline.dataset_id,
                    &name,
                    &(data.len() as i32),
                    &object_id,
                )
                .one()
                .await?;
        }

        transaction.commit().await?;

        Ok(StatusCode::OK)
    } else {
        Err(CustomError::Authentication(
            "You need an API key".to_string(),
        ))
    }
}
