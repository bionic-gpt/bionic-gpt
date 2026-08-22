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
    routes::skills::{Delete, Index, UpdateFile, Upsert, View},
    string_to_visibility,
};
use zip::ZipArchive;

struct SkillForm {
    id: Option<i32>,
    visibility: String,
    upload: Option<UploadedSkill>,
}

#[derive(Default)]
struct SkillMetadata {
    name: Option<String>,
    description: Option<String>,
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

pub async fn action_update_file(
    UpdateFile { team_id, id }: UpdateFile,
    Extension(pool): Extension<Pool>,
    Extension(storage_config): Extension<object_storage::StorageConfig>,
    current_user: Jwt,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, CustomError> {
    let mut relative_path = None;
    let mut content = None;
    while let Some(field) = multipart.next_field().await? {
        match field.name().unwrap_or_default() {
            "relative_path" => relative_path = Some(field.text().await?),
            "content" => content = Some(field.bytes().await?.to_vec()),
            _ => {}
        }
    }
    let relative_path = relative_path.unwrap_or_default();
    let content = content.unwrap_or_default();
    if relative_path.is_empty() || content.len() > 1024 * 1024 {
        return crate::layout::redirect_and_snackbar(
            &View { team_id, id }.to_string(),
            "File path is required and text files must be 1 MiB or smaller",
        );
    }
    if std::str::from_utf8(&content).is_err() {
        return crate::layout::redirect_and_snackbar(
            &View { team_id, id }.to_string(),
            "Only UTF-8 text files can be edited",
        );
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (permissions, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    let file = queries::skills::visible_skill_files()
        .bind(&transaction)
        .all()
        .await?
        .into_iter()
        .find(|file| file.skill_id == id && file.relative_path == relative_path);
    let Some(file) = file else {
        return crate::layout::redirect_and_snackbar(
            &View { team_id, id }.to_string(),
            "File not found",
        );
    };
    if file.is_system {
        return crate::layout::redirect_and_snackbar(
            &View { team_id, id }.to_string(),
            "System skill files cannot be edited",
        );
    }
    let object_id = object_storage::upload(
        &storage_config,
        permissions.user_id,
        team_id_num,
        &relative_path,
        &content,
    )
    .await?;
    queries::skills::update_skill_file()
        .bind(&transaction, &object_id, &id, &relative_path)
        .await?;
    transaction.commit().await?;
    crate::layout::redirect_and_snackbar(&View { team_id, id }.to_string(), "File saved")
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

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (permissions, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let mut visibility = string_to_visibility(&form.visibility);
    if visibility == Visibility::Company && (config.saas || !permissions.is_sys_admin) {
        visibility = Visibility::Team;
    }

    let (extracted_files, uploaded_metadata, fallback_name) = match form.upload {
        Some(upload) => {
            let fallback_name = fallback_skill_name(&upload.file_name);
            match extract_skill_files(upload) {
                Ok(files) => {
                    let metadata = match skill_metadata(&files) {
                        Ok(metadata) => metadata,
                        Err(message) => return redirect_to_index(&index, message),
                    };
                    (Some(files), metadata, fallback_name)
                }
                Err(message) => return redirect_to_index(&index, message),
            }
        }
        None => (None, SkillMetadata::default(), None),
    };

    if form.id.is_none() && extracted_files.is_none() {
        return redirect_to_index(&index, "Upload a SKILL.md file or .zip folder");
    }

    let existing_skill = match form.id {
        Some(id) => Some(
            queries::skills::skill()
                .bind(&transaction, &id)
                .one()
                .await?,
        ),
        None => None,
    };
    let name = uploaded_metadata
        .name
        .or_else(|| existing_skill.as_ref().map(|skill| skill.name.clone()))
        .or(fallback_name)
        .unwrap_or_else(|| "Skill".to_string());
    let description = uploaded_metadata
        .description
        .or_else(|| {
            existing_skill
                .as_ref()
                .map(|skill| skill.description.clone())
        })
        .unwrap_or_default();

    if name.trim().is_empty() {
        return redirect_to_index(&index, "The skill name cannot be empty");
    }

    let skill_id = match form.id {
        Some(id) => {
            queries::skills::update_skill()
                .bind(&transaction, &name, &description, &visibility, &id)
                .await?;
            id
        }
        None => {
            queries::skills::insert_skill()
                .bind(&transaction, &team_id_num, &name, &description, &visibility)
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
        visibility: "Private".to_string(),
        upload: None,
    };

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "id" => {
                form.id = field.text().await?.trim().parse::<i32>().ok();
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

fn fallback_skill_name(file_name: &str) -> Option<String> {
    Path::new(file_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::trim)
        .filter(|name| !name.is_empty() && !name.eq_ignore_ascii_case("skill"))
        .map(ToOwned::to_owned)
}

fn skill_metadata(files: &[SkillFileUpload]) -> Result<SkillMetadata, String> {
    let Some(skill_file) = files.iter().find(|file| file.relative_path == "SKILL.md") else {
        return Err("The skill folder must contain SKILL.md at its root".to_string());
    };

    parse_skill_frontmatter(&skill_file.bytes)
}

fn parse_skill_frontmatter(bytes: &[u8]) -> Result<SkillMetadata, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| "SKILL.md must be valid UTF-8 to read its metadata".to_string())?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let Some(rest) = text.strip_prefix("---\n") else {
        return Ok(SkillMetadata::default());
    };
    let Some((frontmatter, _body)) = rest.split_once("\n---\n") else {
        return Err("SKILL.md frontmatter is not closed".to_string());
    };

    let mut metadata = SkillMetadata::default();
    for line in frontmatter.lines() {
        let Some((key, value)) = line.split_once(':') else {
            return Err("SKILL.md frontmatter contains an invalid line".to_string());
        };
        let value = value.trim().trim_matches(['"', '\'']).trim().to_string();
        if value.is_empty() && matches!(key.trim(), "name" | "description") {
            return Err(format!(
                "SKILL.md frontmatter field `{}` cannot be empty",
                key.trim()
            ));
        }
        match key.trim() {
            "name" => metadata.name = Some(value),
            "description" => metadata.description = Some(value),
            _ => {}
        }
    }

    Ok(metadata)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_skill_frontmatter() {
        let metadata = parse_skill_frontmatter(
            b"---\nname: Dashboard Builder\ndescription: Creates dashboards\n---\n# Instructions",
        )
        .expect("frontmatter should parse");

        assert_eq!(metadata.name.as_deref(), Some("Dashboard Builder"));
        assert_eq!(metadata.description.as_deref(), Some("Creates dashboards"));
    }

    #[test]
    fn accepts_skill_without_frontmatter() {
        let metadata = parse_skill_frontmatter(b"# Legacy skill").expect("legacy skill is valid");

        assert!(metadata.name.is_none());
        assert!(metadata.description.is_none());
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        assert!(parse_skill_frontmatter(b"---\nname: Broken\n# Instructions").is_err());
    }

    #[test]
    fn derives_fallback_name_from_zip_filename() {
        assert_eq!(
            fallback_skill_name("dashboard-builder.zip").as_deref(),
            Some("dashboard-builder")
        );
        assert_eq!(fallback_skill_name("SKILL.md"), None);
    }
}
