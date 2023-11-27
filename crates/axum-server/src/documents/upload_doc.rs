use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Multipart, Path},
    response::IntoResponse,
};
use db::queries;
use db::Pool;

pub async fn upload(
    Path((team_id, dataset_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    mut files: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

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

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::documents::index_route(team_id, dataset_id),
        "Document Uploaded",
    )
}
