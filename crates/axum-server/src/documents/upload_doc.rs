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
        // this is the name which is sent in formdata from frontend or whoever called the api, i am
        // using it as category, we can get the filename from file data
        let _category = file.name().unwrap().to_string();
        // name of the file with extention
        let name = file.file_name().unwrap().to_string();
        // file data
        let data = file.bytes().await.unwrap();

        tracing::info!("Sending document to unstructured");

        let unstructured_data =
            crate::unstructured::call_unstructured_api(data.to_vec(), &name).await?;

        let text: Vec<String> = unstructured_data.into_iter().map(|u| u.text).collect();
        let text = text.join(" ");

        tracing::info!("Creating document in postgres");

        let document_id = queries::documents::insert()
            .bind(&transaction, &dataset_id, &name)
            .one()
            .await?;

        tracing::info!("Inserting text batches");

        for text_bytes in text.as_bytes().chunks(1024) {
            let text_utf8 = String::from_utf8_lossy(text_bytes).to_string();
            transaction
                .execute(
                    "
                    INSERT INTO embeddings (
                        document_id,
                        text
                    ) 
                    VALUES 
                        ($1, $2)",
                    &[&document_id, &text_utf8],
                )
                .await?;
        }
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_components::routes::documents::index_route(team_id, dataset_id),
        "Document Uploaded and Embeddings Created",
    )
}
