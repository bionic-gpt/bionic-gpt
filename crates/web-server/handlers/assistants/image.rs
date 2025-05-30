use crate::{CustomError, Jwt};
use axum::body::Body;
use axum::body::Bytes;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::response::Response;
use db::authz;
use db::Pool;
use web_pages::routes::prompts::Image;

pub async fn image(
    Image { team_id, id }: Image,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let object = object_storage::get(pool, id).await?;

    if let Some(bytes) = object.object_data {
        let bytes = Bytes::from(bytes);

        Ok(Response::builder()
            .header("Content-Type", object.mime_type) // Change this to match your image type
            .body(Body::from(bytes))
            .unwrap())
    } else {
        Err(CustomError::Database(
            "No object data in that storage".to_string(),
        ))
    }
}
