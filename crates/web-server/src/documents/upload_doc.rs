use super::super::{CustomError, Jwt};
use axum::{
    extract::{Extension, Multipart},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::documents::Upload;

pub async fn upload(
    Upload {
        team_id,
        dataset_id,
    }: Upload,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    mut files: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    while let Some(file) = files.next_field().await.unwrap() {
        // name of the file with extention
        let name = file.file_name().unwrap().to_string();
        // file data
        let data = file.bytes().await.unwrap().to_vec();

        let _document_id = queries::documents::insert()
            .bind(
                &transaction,
                &dataset_id,
                &name,
                &data,
                &(data.len() as i32),
            )
            .one()
            .await?;
    }

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(
        &web_pages::routes::documents::Index {
            team_id,
            dataset_id,
        }
        .to_string(),
        "Document Uploaded",
    )
}
