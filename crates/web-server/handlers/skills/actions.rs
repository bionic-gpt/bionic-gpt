use crate::config::Config;
use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Multipart},
    response::{IntoResponse, Response},
};
use db::authz;
use db::queries;
use db::{Pool, Visibility};
use std::collections::BTreeSet;
use std::io::{Cursor, Read};
use std::path::{Component, Path, PathBuf};
use web_pages::{
    routes::skills::{Delete, Index, Upsert},
    string_to_visibility,
};
use zip::ZipArchive;

struct SkillForm {
    id: Option<i32>,
    name: String,
    description: String,
    visibility: String,
    upload: Option<UploadedSkill>,
}

struct UploadedSkill {
    file_name: String,
    bytes: Vec<u8>,
}

struct SkillFileUpload {
    relative_path: String,
    bytes: Vec<u8>,
}

pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (_permissions, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    queries::skills::delete_skill()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::skills::Index { team_id }.to_string(),
        "Skill Deleted",
    )
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    Extension(storage_config): Extension<object_storage::StorageConfig>,
    current_user: Jwt,
    multipart: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    let index = Index {
        team_id: team_id.clone(),
    }
    .to_string();
    let form = parse_skill_form(multipart).await?;

    if form.name.trim().is_empty() {
        return redirect_to_index(&index, "Skill name is required");
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (permissions, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let mut visibility = string_to_visibility(&form.visibility);
    if visibility == Visibility::Company && (config.saas || !permissions.is_sys_admin) {
        visibility = Visibility::Team;
    }

    let extracted_files = match form.upload {
        Some(upload) => match extract_skill_files(upload) {
            Ok(files) => Some(files),
            Err(message) => return redirect_to_index(&index, message),
        },
        None if form.id.is_none() => {
            return redirect_to_index(&index, "Upload a SKILL.md file or .zip folder");
        }
        None => None,
    };

    let skill_id = match form.id {
        Some(id) => {
            queries::skills::update_skill()
                .bind(
                    &transaction,
                    &form.name,
                    &form.description,
                    &visibility,
                    &id,
                )
                .await?;
            id
        }
        None => {
            queries::skills::insert_skill()
                .bind(
                    &transaction,
                    &team_id_num,
                    &form.name,
                    &form.description,
                    &visibility,
                )
                .one()
                .await?
        }
    };

    if let Some(files) = extracted_files {
        queries::skills::delete_skill_files()
            .bind(&transaction, &skill_id)
            .await?;

        for file in files {
            let object_id = object_storage::upload(
                &storage_config,
                permissions.user_id,
                team_id_num,
                &file.relative_path,
                &file.bytes,
            )
            .await?;

            queries::skills::insert_skill_file()
                .bind(&transaction, &skill_id, &object_id, &file.relative_path)
                .await?;
        }
    }

    transaction.commit().await?;

    let message = if form.id.is_some() {
        "Skill Updated"
    } else {
        "Skill Created"
    };
    redirect_to_index(&index, message)
}

fn redirect_to_index(index: &str, message: impl Into<String>) -> Result<Response, CustomError> {
    crate::layout::redirect_and_snackbar(index, message).map(IntoResponse::into_response)
}

async fn parse_skill_form(mut multipart: Multipart) -> Result<SkillForm, CustomError> {
    let mut form = SkillForm {
        id: None,
        name: String::new(),
        description: String::new(),
        visibility: "Private".to_string(),
        upload: None,
    };

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "id" => {
                form.id = field.text().await?.trim().parse::<i32>().ok();
            }
            "name" => {
                form.name = field.text().await?.trim().to_string();
            }
            "description" => {
                form.description = field.text().await?.trim().to_string();
            }
            "visibility" => {
                form.visibility = field.text().await?;
            }
            "payload" => {
                let file_name = field.file_name().unwrap_or_default().to_string();
                let bytes = field.bytes().await?.to_vec();
                if !file_name.is_empty() && !bytes.is_empty() {
                    form.upload = Some(UploadedSkill { file_name, bytes });
                }
            }
            _ => {}
        }
    }

    Ok(form)
}

fn extract_skill_files(upload: UploadedSkill) -> Result<Vec<SkillFileUpload>, String> {
    if upload.file_name.ends_with(".zip") {
        extract_zip_skill_files(&upload.bytes)
    } else if Path::new(&upload.file_name)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("SKILL.md")
    {
        Ok(vec![SkillFileUpload {
            relative_path: "SKILL.md".to_string(),
            bytes: upload.bytes,
        }])
    } else {
        Err("Upload a file named SKILL.md or a .zip folder".to_string())
    }
}

fn extract_zip_skill_files(bytes: &[u8]) -> Result<Vec<SkillFileUpload>, String> {
    let reader = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(reader).map_err(|_| "The uploaded .zip could not be opened".to_string())?;
    let mut raw_files = Vec::new();

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|_| "The uploaded .zip could not be read".to_string())?;

        if file.is_dir() {
            continue;
        }

        let enclosed_name = file
            .enclosed_name()
            .ok_or_else(|| "The .zip contains an unsafe file path".to_string())?;
        let relative_path = normalize_zip_path(&enclosed_name)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .map_err(|_| "The uploaded .zip file contents could not be read".to_string())?;

        if !contents.is_empty() {
            raw_files.push((relative_path, contents));
        }
    }

    let files = strip_common_root(raw_files)
        .into_iter()
        .map(|(relative_path, bytes)| SkillFileUpload {
            relative_path,
            bytes,
        })
        .collect::<Vec<_>>();

    if files.is_empty() {
        return Err("The .zip does not contain any skill files".to_string());
    }

    if !files.iter().any(|file| file.relative_path == "SKILL.md") {
        return Err("The skill folder must contain SKILL.md at its root".to_string());
    }

    Ok(files)
}

fn normalize_zip_path(path: &Path) -> Result<String, String> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            _ => return Err("The .zip contains an unsafe file path".to_string()),
        }
    }

    normalized
        .to_str()
        .map(|path| path.replace('\\', "/"))
        .filter(|path| !path.is_empty())
        .ok_or_else(|| "The .zip contains an invalid file path".to_string())
}

fn strip_common_root(files: Vec<(String, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
    let roots = files
        .iter()
        .filter_map(|(path, _)| path.split('/').next())
        .collect::<BTreeSet<_>>();

    if roots.len() != 1 {
        return files;
    }

    let root = roots.into_iter().next().unwrap_or_default().to_string();
    let all_nested = files.iter().all(|(path, _)| {
        path.strip_prefix(&root)
            .is_some_and(|rest| rest.starts_with('/'))
    });

    if !all_nested {
        return files;
    }

    files
        .into_iter()
        .filter_map(|(path, bytes)| {
            path.strip_prefix(&root)
                .and_then(|path| path.strip_prefix('/'))
                .filter(|path| !path.is_empty())
                .map(|path| (path.to_string(), bytes))
        })
        .collect()
}
