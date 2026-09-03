use crate::{CustomError, Jwt};
use axum::body::Body;
use axum::extract::Extension;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use db::{authz, queries, Pool};
use web_pages::routes::console::GeneratedOutputDownload;

pub async fn generated_output_download(
    GeneratedOutputDownload { team_id, id }: GeneratedOutputDownload,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (_rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let output = queries::generated_outputs::get_content()
        .bind(&transaction, &id)
        .one()
        .await?;

    let file_name = download_file_name(&output.file_name);
    let content_disposition = format!("attachment; filename=\"{file_name}\"");

    Ok(Response::builder()
        .header(CONTENT_TYPE, output.mime_type)
        .header(CONTENT_DISPOSITION, content_disposition)
        .body(Body::from(output.object_data))
        .unwrap())
}

fn download_file_name(file_name: &str) -> String {
    let name = file_name
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("download")
        .chars()
        .filter(|character| !character.is_control() && *character != '"')
        .collect::<String>();

    if name.is_empty() {
        "download".to_string()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::download_file_name;

    #[test]
    fn keeps_safe_file_names() {
        assert_eq!(download_file_name("report.pdf"), "report.pdf");
    }

    #[test]
    fn removes_path_and_header_unsafe_characters() {
        assert_eq!(download_file_name("../report\".pdf\n"), "report.pdf");
    }

    #[test]
    fn supplies_a_fallback_name() {
        assert_eq!(download_file_name("\n\""), "download");
    }
}
