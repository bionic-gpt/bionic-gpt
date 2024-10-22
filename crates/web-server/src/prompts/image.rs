use super::super::{Authentication, CustomError};
use axum::body::Body;
use axum::body::Bytes;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::response::Response;
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::prompts::Image;

pub async fn image(
    Image { team_id, id }: Image,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let image = queries::prompts::image()
        .bind(&transaction, &id, &team_id)
        .one()
        .await?;
    // Convert stream to axum HTTP body
    let bytes = Bytes::from(image);

    Ok(Response::builder()
        .header("Content-Type", "image/png") // Change this to match your image type
        .body(Body::from(bytes))
        .unwrap())
}
