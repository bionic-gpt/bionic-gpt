use db::{queries::object_storage, ObjectStorage, Pool, TokioPostgresError};
use image::imageops::FilterType;

#[derive(Debug)]
pub enum StorageError {
    DatabaseError(String),
    InvalidInput(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StorageError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            StorageError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl From<TokioPostgresError> for StorageError {
    fn from(err: TokioPostgresError) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

impl From<db::PoolError> for StorageError {
    fn from(err: db::PoolError) -> StorageError {
        StorageError::DatabaseError(err.to_string())
    }
}

pub async fn upload(
    pool: Pool,
    user_id: i32,
    team_id: i32,
    file_name: &str,
    bytes: &[u8],
) -> Result<i32, StorageError> {
    if file_name.is_empty() || bytes.is_empty() {
        return Err(StorageError::InvalidInput(
            "File name and bytes cannot be empty".into(),
        ));
    }

    let object_name = file_name.to_string();
    let mime_type = mime_guess::from_path(file_name)
        .first_or_octet_stream()
        .to_string();
    let file_size = bytes.len() as i64;
    let file_hash = format!("{:x}", md5::compute(bytes));

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let id = object_storage::insert()
        .bind(
            &transaction,
            &object_name,
            &team_id,
            &bytes,
            &mime_type,
            &file_name,
            &file_size,
            &file_hash,
            &user_id,
        )
        .one()
        .await?;

    transaction.commit().await?;

    Ok(id)
}

pub async fn image_upload(
    pool: Pool,
    user_id: i32,
    team_id: i32,
    file_name: &str,
    bytes: &[u8],
    image_size: Option<(u32, u32)>,
) -> Result<i32, StorageError> {
    let resized_bytes = resize_image(bytes, image_size)?;
    let id = upload(pool, user_id, team_id, file_name, &resized_bytes).await?;
    Ok(id)
}

pub fn resize_image(bytes: &[u8], size: Option<(u32, u32)>) -> Result<Vec<u8>, StorageError> {
    if let Some((width, height)) = size {
        // Load the image from bytes
        let img = image::load_from_memory(bytes)
            .map_err(|e| StorageError::InvalidInput(e.to_string()))?;

        // Resize the image
        let resized_img = img.resize(width, height, FilterType::Lanczos3);

        // Determine the original format based on the input bytes
        let format =
            image::guess_format(bytes).map_err(|e| StorageError::InvalidInput(e.to_string()))?;

        // Convert the resized image back to bytes in the original format
        let mut output = Vec::new();
        resized_img
            .write_to(&mut std::io::Cursor::new(&mut output), format)
            .map_err(|e| StorageError::InvalidInput(e.to_string()))?;

        Ok(output)
    } else {
        Ok(bytes.to_vec()) // Return original bytes if no size is specified
    }
}

pub async fn get(pool: Pool, id: i32) -> Result<ObjectStorage, StorageError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let object = object_storage::get().bind(&transaction, &id).one().await?;

    Ok(object)
}
