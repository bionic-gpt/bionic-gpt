use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form, Multipart},
    response::{Html, IntoResponse},
};
use db::{authz, queries, Json, OpenapiSpecCategory, Pool};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Cursor, Read};
use std::path::{Component, Path};
use validator::Validate;
use web_pages::openapi_specs::upsert::OpenapiSpecForm;
use web_pages::routes::openapi_specs::{Delete, Import, Upsert};
use zip::ZipArchive;

use super::super::integrations::helpers::parse_openapi_spec_json_value;

pub(super) const MAX_UPLOAD_BYTES: usize = 50 * 1024 * 1024;
const MAX_SPEC_BYTES: usize = 5 * 1024 * 1024;
const MAX_TOTAL_SPEC_BYTES: usize = 100 * 1024 * 1024;
const MAX_SPEC_COUNT: usize = 500;

struct UploadForm {
    category: String,
    is_active: bool,
    payload: Option<UploadedFile>,
}

struct UploadedFile {
    file_name: String,
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct SpecFile {
    source_name: String,
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct PreparedSpec {
    source_name: String,
    slug: String,
    title: String,
    description: String,
    logo_url: String,
    spec: serde_json::Value,
}

fn spec_string_at<'a>(spec: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    spec.get("info")?
        .get(key)?
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn spec_nested_string_at<'a>(
    spec: &'a serde_json::Value,
    first_key: &str,
    second_key: &str,
) -> Option<&'a str> {
    spec.get("info")?
        .get(first_key)?
        .get(second_key)?
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn slugify(value: &str) -> Option<String> {
    let mut slug = String::new();
    let mut previous_was_separator = false;

    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            previous_was_separator = false;
        } else if !previous_was_separator && !slug.is_empty() {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        None
    } else {
        Some(slug)
    }
}

fn derive_slug_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_string_at(spec, "x-bionic-slug")
        .or_else(|| spec_string_at(spec, "bionic-slug"))
        .or_else(|| spec_string_at(spec, "title"))
        .and_then(slugify)
}

fn derive_title_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_string_at(spec, "title").map(str::to_string)
}

fn derive_description_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_string_at(spec, "description").map(str::to_string)
}

fn derive_logo_url_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_nested_string_at(spec, "x-logo", "url")
        .or_else(|| spec_nested_string_at(spec, "logo", "url"))
        .map(str::to_string)
}

fn parse_category(category: &str) -> OpenapiSpecCategory {
    match category {
        "WebSearch" => OpenapiSpecCategory::WebSearch,
        _ => OpenapiSpecCategory::Application,
    }
}

fn should_auto_select_system_spec(category: OpenapiSpecCategory, is_active: bool) -> bool {
    category == OpenapiSpecCategory::WebSearch && is_active
}

async fn parse_upload_form(mut multipart: Multipart) -> Result<UploadForm, CustomError> {
    let mut form = UploadForm {
        category: "Application".to_string(),
        is_active: false,
        payload: None,
    };

    while let Some(field) = multipart.next_field().await? {
        match field.name().unwrap_or_default() {
            "category" => form.category = field.text().await?.trim().to_string(),
            "is_active" => form.is_active = field.text().await?.trim() == "true",
            "payload" => {
                let file_name = field.file_name().unwrap_or_default().to_string();
                let bytes = field.bytes().await?.to_vec();
                if !file_name.is_empty() && !bytes.is_empty() {
                    form.payload = Some(UploadedFile { file_name, bytes });
                }
            }
            _ => {}
        }
    }

    Ok(form)
}

fn supported_spec_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "json" | "yaml" | "yml"
            )
        })
        .unwrap_or(false)
}

fn normalized_zip_path(path: &Path) -> Result<String, String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => parts.push(
                part.to_str()
                    .ok_or_else(|| "The ZIP contains a non-UTF-8 file path".to_string())?,
            ),
            _ => return Err("The ZIP contains an unsafe file path".to_string()),
        }
    }

    if parts.is_empty() {
        Err("The ZIP contains an invalid file path".to_string())
    } else {
        Ok(parts.join("/"))
    }
}

fn extract_zip_specs(bytes: &[u8]) -> Result<Vec<SpecFile>, String> {
    let mut archive = ZipArchive::new(Cursor::new(bytes))
        .map_err(|_| "The uploaded ZIP could not be opened".to_string())?;
    let mut files = Vec::new();
    let mut paths = BTreeSet::new();
    let mut total_bytes = 0usize;

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|_| "The uploaded ZIP could not be read".to_string())?;
        if entry.is_dir() {
            continue;
        }

        let enclosed_name = entry
            .enclosed_name()
            .ok_or_else(|| "The ZIP contains an unsafe file path".to_string())?;
        let source_name = normalized_zip_path(&enclosed_name)?;
        if !paths.insert(source_name.clone()) {
            return Err(format!(
                "The ZIP contains the path '{source_name}' more than once"
            ));
        }
        if !supported_spec_extension(&enclosed_name) {
            continue;
        }
        if files.len() >= MAX_SPEC_COUNT {
            return Err(format!(
                "The ZIP contains more than {MAX_SPEC_COUNT} OpenAPI spec files"
            ));
        }
        if entry.size() > MAX_SPEC_BYTES as u64 {
            return Err(format!(
                "'{source_name}' exceeds the 5 MiB uncompressed file limit"
            ));
        }

        let mut contents = Vec::new();
        entry
            .by_ref()
            .take((MAX_SPEC_BYTES + 1) as u64)
            .read_to_end(&mut contents)
            .map_err(|_| format!("'{source_name}' could not be read"))?;
        if contents.len() > MAX_SPEC_BYTES {
            return Err(format!(
                "'{source_name}' exceeds the 5 MiB uncompressed file limit"
            ));
        }
        total_bytes = total_bytes
            .checked_add(contents.len())
            .ok_or_else(|| "The ZIP is too large after decompression".to_string())?;
        if total_bytes > MAX_TOTAL_SPEC_BYTES {
            return Err("The ZIP exceeds the 100 MiB uncompressed limit".to_string());
        }

        files.push(SpecFile {
            source_name,
            bytes: contents,
        });
    }

    if files.is_empty() {
        return Err("The ZIP does not contain any JSON or YAML spec files".to_string());
    }
    files.sort_by(|left, right| left.source_name.cmp(&right.source_name));
    Ok(files)
}

fn extract_spec_files(upload: UploadedFile) -> Result<Vec<SpecFile>, String> {
    let path = Path::new(&upload.file_name);
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if extension == "zip" {
        return extract_zip_specs(&upload.bytes);
    }
    if !supported_spec_extension(path) {
        return Err("Upload a .json, .yaml, .yml, or .zip file".to_string());
    }
    if upload.bytes.len() > MAX_SPEC_BYTES {
        return Err("The uploaded spec exceeds the 5 MiB file limit".to_string());
    }

    Ok(vec![SpecFile {
        source_name: upload.file_name,
        bytes: upload.bytes,
    }])
}

fn prepare_specs(files: Vec<SpecFile>) -> Result<Vec<PreparedSpec>, Vec<String>> {
    let mut prepared = Vec::new();
    let mut errors = Vec::new();

    for file in files {
        let text = match std::str::from_utf8(&file.bytes) {
            Ok(text) => text,
            Err(_) => {
                errors.push(format!("{}: file is not valid UTF-8", file.source_name));
                continue;
            }
        };
        let parsed_spec = match parse_openapi_spec_json_value(text.trim()) {
            Ok(spec) => spec,
            Err(error) => {
                errors.push(format!("{}: {error}", file.source_name));
                continue;
            }
        };
        let Some(slug) = derive_slug_from_spec(&parsed_spec) else {
            errors.push(format!(
                "{}: could not derive a slug from info.x-bionic-slug, info.bionic-slug, or info.title",
                file.source_name
            ));
            continue;
        };
        let Some(title) = derive_title_from_spec(&parsed_spec) else {
            errors.push(format!(
                "{}: the OpenAPI spec must contain a non-empty info.title",
                file.source_name
            ));
            continue;
        };

        prepared.push(PreparedSpec {
            source_name: file.source_name,
            slug,
            title,
            description: derive_description_from_spec(&parsed_spec).unwrap_or_default(),
            logo_url: derive_logo_url_from_spec(&parsed_spec).unwrap_or_default(),
            spec: parsed_spec,
        });
    }

    let mut sources_by_slug = BTreeMap::<String, Vec<String>>::new();
    for spec in &prepared {
        sources_by_slug
            .entry(spec.slug.clone())
            .or_default()
            .push(spec.source_name.clone());
    }
    for (slug, sources) in sources_by_slug {
        if sources.len() > 1 {
            errors.push(format!(
                "Slug '{slug}' is produced by multiple files: {}",
                sources.join(", ")
            ));
        }
    }

    if errors.is_empty() {
        Ok(prepared)
    } else {
        Err(errors)
    }
}

fn upload_error_message(errors: impl IntoIterator<Item = String>) -> String {
    let details = errors
        .into_iter()
        .map(|error| format!("- {error}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("OpenAPI spec upload failed:\n{details}")
}

pub async fn action_import(
    Import { team_id }: Import,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    multipart: Multipart,
) -> Result<axum::response::Response, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let existing_specs = queries::openapi_specs::list()
        .bind(&transaction)
        .all()
        .await?;
    let form = parse_upload_form(multipart).await?;
    let Some(payload) = form.payload else {
        let error = upload_error_message(["Choose a JSON, YAML, or ZIP file".to_string()]);
        let html = web_pages::openapi_specs::page::page(team_id, rbac, existing_specs, Some(error));
        return Ok(Html(html).into_response());
    };

    let files = match extract_spec_files(payload) {
        Ok(files) => files,
        Err(error) => {
            let html = web_pages::openapi_specs::page::page(
                team_id,
                rbac,
                existing_specs,
                Some(upload_error_message([error])),
            );
            return Ok(Html(html).into_response());
        }
    };
    let prepared = match prepare_specs(files) {
        Ok(prepared) => prepared,
        Err(errors) => {
            let html = web_pages::openapi_specs::page::page(
                team_id,
                rbac,
                existing_specs,
                Some(upload_error_message(errors)),
            );
            return Ok(Html(html).into_response());
        }
    };

    let existing_slugs = existing_specs
        .iter()
        .map(|spec| spec.slug.as_str())
        .collect::<BTreeSet<_>>();
    let conflicts = prepared
        .iter()
        .filter(|spec| existing_slugs.contains(spec.slug.as_str()))
        .map(|spec| format!("{}: slug '{}' already exists", spec.source_name, spec.slug))
        .collect::<Vec<_>>();
    if !conflicts.is_empty() {
        let html = web_pages::openapi_specs::page::page(
            team_id,
            rbac,
            existing_specs,
            Some(upload_error_message(conflicts)),
        );
        return Ok(Html(html).into_response());
    }

    let category = parse_category(&form.category);
    let imported_count = prepared.len();
    let auto_select =
        imported_count == 1 && should_auto_select_system_spec(category, form.is_active);
    let mut imported_spec_id = None;

    for spec in prepared {
        let description = (!spec.description.is_empty()).then_some(spec.description.as_str());
        let logo_url = (!spec.logo_url.is_empty()).then_some(spec.logo_url.as_str());
        let spec_json = Json(spec.spec);
        let result = queries::openapi_specs::insert()
            .bind(
                &transaction,
                &spec.slug,
                &spec.title,
                &description,
                &spec_json,
                &logo_url,
                &category,
                &form.is_active,
            )
            .one()
            .await;

        match result {
            Ok(spec_id) => imported_spec_id = Some(spec_id),
            Err(error) => {
                if error
                    .as_db_error()
                    .is_some_and(|db_error| db_error.code().code() == "23505")
                {
                    let html = web_pages::openapi_specs::page::page(
                        team_id,
                        rbac,
                        existing_specs,
                        Some(upload_error_message([format!(
                            "{}: slug '{}' was created by another import",
                            spec.source_name, spec.slug
                        )])),
                    );
                    return Ok(Html(html).into_response());
                }
                return Err(CustomError::from(error));
            }
        }
    }

    if auto_select {
        queries::openapi_spec_selections::set_selection()
            .bind(
                &transaction,
                &OpenapiSpecCategory::WebSearch,
                &imported_spec_id.expect("single imported spec has an id"),
            )
            .await?;
    }

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        &format!(
            "Imported {imported_count} OpenAPI {}",
            if imported_count == 1 { "spec" } else { "specs" }
        ),
    )
    .into_response())
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(mut form): Form<OpenapiSpecForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    // Trim whitespace from inputs
    form.category = form.category.trim().to_string();
    form.spec = form.spec.trim().to_string();

    if let Err(validation) = form.validate() {
        form.error = Some(format!("Validation error: {}", validation));
        let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
        return Ok(Html(html).into_response());
    }

    let parsed_spec = match parse_openapi_spec_json_value(&form.spec) {
        Ok(value) => value,
        Err(error) => {
            form.error = Some(error);
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };
    form.slug = match derive_slug_from_spec(&parsed_spec) {
        Some(slug) => slug,
        None => {
            form.error = Some(
                "Could not derive a slug from the OpenAPI spec. Add info.x-bionic-slug or a non-empty info.title."
                    .to_string(),
            );
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };
    form.title = match derive_title_from_spec(&parsed_spec) {
        Some(title) => title,
        None => {
            form.error = Some(
                "Could not derive a title from the OpenAPI spec. Add a non-empty info.title."
                    .to_string(),
            );
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };
    form.description = derive_description_from_spec(&parsed_spec).unwrap_or_default();
    form.logo_url = derive_logo_url_from_spec(&parsed_spec).unwrap_or_default();

    let description_param = if form.description.is_empty() {
        None
    } else {
        Some(form.description.as_str())
    };

    let logo_url_param = if form.logo_url.is_empty() {
        None
    } else {
        Some(form.logo_url.as_str())
    };

    let category = parse_category(&form.category);
    let spec_json = Json(parsed_spec);

    if let Some(id) = form.id {
        let existing = queries::openapi_specs::by_id()
            .bind(&transaction, &id)
            .one()
            .await?;
        if existing.is_system {
            return Err(CustomError::Authorization);
        }
    }

    let result: Result<i32, db::TokioPostgresError> = if let Some(id) = form.id {
        queries::openapi_specs::update()
            .bind(
                &transaction,
                &form.slug,
                &form.title,
                &description_param,
                &spec_json,
                &logo_url_param,
                &category,
                &form.is_active,
                &id,
            )
            .await
            .map(|_| id)
    } else {
        queries::openapi_specs::insert()
            .bind(
                &transaction,
                &form.slug,
                &form.title,
                &description_param,
                &spec_json,
                &logo_url_param,
                &category,
                &form.is_active,
            )
            .one()
            .await
    };

    let spec_id = match result {
        Ok(spec_id) => spec_id,
        Err(error) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code().code() == "23505" {
                    form.error =
                        Some("Slug already exists. Please choose another one.".to_string());
                    let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
                    return Ok(Html(html).into_response());
                }
            }
            return Err(CustomError::from(error));
        }
    };

    if should_auto_select_system_spec(category, form.is_active) {
        queries::openapi_spec_selections::set_selection()
            .bind(&transaction, &OpenapiSpecCategory::WebSearch, &spec_id)
            .await?;
    }

    transaction.commit().await?;

    let message = if form.id.is_some() {
        "OpenAPI spec updated"
    } else {
        "OpenAPI spec created"
    };

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        message,
    )
    .into_response())
}
pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let spec = queries::openapi_specs::by_id()
        .bind(&transaction, &id)
        .one()
        .await?;
    if spec.is_system {
        return Err(CustomError::Authorization);
    }

    queries::openapi_specs::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        "OpenAPI spec deleted",
    )
    .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    fn spec_bytes(title: &str, slug: &str) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "openapi": "3.1.0",
            "info": {
                "title": title,
                "version": "1.0.0",
                "x-bionic-slug": slug
            },
            "paths": {
                "/health": {
                    "get": {
                        "operationId": "health",
                        "responses": {"200": {"description": "OK"}}
                    }
                }
            }
        }))
        .unwrap()
    }

    fn zip_bytes(files: &[(&str, Vec<u8>)]) -> Vec<u8> {
        let cursor = Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(cursor);
        for (path, contents) in files {
            writer
                .start_file(*path, SimpleFileOptions::default())
                .unwrap();
            writer.write_all(contents).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn extracts_supported_specs_from_nested_zip() {
        let archive = zip_bytes(&[
            ("specs/calendar.JSON", spec_bytes("Calendar", "calendar")),
            ("specs/mail.yaml", spec_bytes("Mail", "mail")),
            ("README.md", b"ignored".to_vec()),
        ]);

        let files = extract_zip_specs(&archive).unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].source_name, "specs/calendar.JSON");
        assert_eq!(files[1].source_name, "specs/mail.yaml");
    }

    #[test]
    fn rejects_zip_without_specs() {
        let archive = zip_bytes(&[("README.md", b"nothing to import".to_vec())]);

        assert_eq!(
            extract_zip_specs(&archive).unwrap_err(),
            "The ZIP does not contain any JSON or YAML spec files"
        );
    }

    #[test]
    fn rejects_unsafe_zip_paths() {
        let archive = zip_bytes(&[("../calendar.json", spec_bytes("Calendar", "calendar"))]);

        assert_eq!(
            extract_zip_specs(&archive).unwrap_err(),
            "The ZIP contains an unsafe file path"
        );
    }

    #[test]
    fn accepts_single_yaml_upload() {
        let files = extract_spec_files(UploadedFile {
            file_name: "calendar.YAML".to_string(),
            bytes: spec_bytes("Calendar", "calendar"),
        })
        .unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].source_name, "calendar.YAML");
    }

    #[test]
    fn reports_invalid_file_with_its_archive_path() {
        let error = prepare_specs(vec![SpecFile {
            source_name: "nested/broken.yaml".to_string(),
            bytes: b"not: [valid".to_vec(),
        }])
        .unwrap_err();

        assert_eq!(error.len(), 1);
        assert!(error[0].starts_with("nested/broken.yaml:"));
    }

    #[test]
    fn rejects_duplicate_slugs_within_upload() {
        let error = prepare_specs(vec![
            SpecFile {
                source_name: "one.json".to_string(),
                bytes: spec_bytes("One", "shared"),
            },
            SpecFile {
                source_name: "two.yaml".to_string(),
                bytes: spec_bytes("Two", "shared"),
            },
        ])
        .unwrap_err();

        assert_eq!(error.len(), 1);
        assert!(error[0].contains("Slug 'shared' is produced by multiple files"));
        assert!(error[0].contains("one.json"));
        assert!(error[0].contains("two.yaml"));
    }

    #[test]
    fn derive_slug_prefers_x_bionic_slug() {
        let spec = json!({
            "info": {
                "title": "Calendar API",
                "bionic-slug": "calendar",
                "x-bionic-slug": "websearch"
            }
        });

        assert_eq!(derive_slug_from_spec(&spec), Some("websearch".to_string()));
    }

    #[test]
    fn derive_slug_falls_back_to_legacy_bionic_slug() {
        let spec = json!({
            "info": {
                "title": "Calendar API",
                "bionic-slug": "google-calendar"
            }
        });

        assert_eq!(
            derive_slug_from_spec(&spec),
            Some("google-calendar".to_string())
        );
    }

    #[test]
    fn derive_slug_slugifies_title() {
        let spec = json!({
            "info": {
                "title": "Enterprise Email API"
            }
        });

        assert_eq!(
            derive_slug_from_spec(&spec),
            Some("enterprise-email-api".to_string())
        );
    }

    #[test]
    fn derive_slug_returns_none_for_unusable_values() {
        let spec = json!({
            "info": {
                "title": "  !!!  "
            }
        });

        assert_eq!(derive_slug_from_spec(&spec), None);
    }

    #[test]
    fn derive_title_reads_openapi_info_title() {
        let spec = json!({
            "info": {
                "title": "Enterprise Email API"
            }
        });

        assert_eq!(
            derive_title_from_spec(&spec),
            Some("Enterprise Email API".to_string())
        );
    }

    #[test]
    fn derive_title_rejects_blank_title() {
        let spec = json!({
            "info": {
                "title": "  "
            }
        });

        assert_eq!(derive_title_from_spec(&spec), None);
    }

    #[test]
    fn derive_description_reads_openapi_info_description() {
        let spec = json!({
            "info": {
                "description": "Inbox eval API."
            }
        });

        assert_eq!(
            derive_description_from_spec(&spec),
            Some("Inbox eval API.".to_string())
        );
    }

    #[test]
    fn derive_logo_prefers_x_logo() {
        let spec = json!({
            "info": {
                "x-logo": {"url": "https://example.com/x-logo.svg"},
                "logo": {"url": "https://example.com/logo.svg"}
            }
        });

        assert_eq!(
            derive_logo_url_from_spec(&spec),
            Some("https://example.com/x-logo.svg".to_string())
        );
    }

    #[test]
    fn derive_logo_falls_back_to_legacy_logo() {
        let spec = json!({
            "info": {
                "logo": {"url": "https://example.com/logo.svg"}
            }
        });

        assert_eq!(
            derive_logo_url_from_spec(&spec),
            Some("https://example.com/logo.svg".to_string())
        );
    }

    #[test]
    fn auto_selects_active_web_search_specs() {
        assert!(should_auto_select_system_spec(
            OpenapiSpecCategory::WebSearch,
            true
        ));
    }

    #[test]
    fn does_not_auto_select_inactive_or_application_specs() {
        assert!(!should_auto_select_system_spec(
            OpenapiSpecCategory::WebSearch,
            false
        ));
        assert!(!should_auto_select_system_spec(
            OpenapiSpecCategory::Application,
            true
        ));
    }
}
