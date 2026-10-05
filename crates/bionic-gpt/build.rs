use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Serialize)]
struct Integration {
    slug: String,
    title: String,
    description: String,
    logo_url: String,
    logo_data_uri: String,
    source_url: String,
    filename: String,
}

fn generate_integrations(manifest_dir: &Path) {
    let source_dir = manifest_dir.join("content/docs/integrations/specs");
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut paths = fs::read_dir(&source_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", source_dir.display()))
        .map(|entry| {
            entry
                .expect("failed to read integration directory entry")
                .path()
        })
        .filter(|path| {
            path.is_file()
                && matches!(
                    path.extension().and_then(|extension| extension.to_str()),
                    Some("yaml" | "yml" | "json")
                )
        })
        .collect::<Vec<_>>();
    paths.sort();

    let mut integrations = Vec::new();
    let docs = String::from(
        "# Curated OpenAPI integrations\n\nBionic provides a curated collection of OpenAPI specifications for common business systems. Upload a specification to make its API operations available as tools; the description in each specification helps the model choose and use those tools.\n\n[Download all integrations (.zip)](/docs/integrations/curated-integrations.zip)\n",
    );

    for path in paths {
        println!("cargo:rerun-if-changed={}", path.display());
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let parsed = oas3::from_json(&source)
            .map_err(|error| error.to_string())
            .or_else(|_| oas3::from_yaml(&source).map_err(|error| error.to_string()))
            .unwrap_or_else(|error| panic!("invalid integration spec {}: {error}", path.display()));
        for (api_path, method, operation) in parsed.operations() {
            if operation.operation_id.as_deref().is_none_or(str::is_empty) {
                panic!(
                    "{}: every operation needs operationId (missing on {} {})",
                    path.display(),
                    method,
                    api_path
                );
            }
        }
        let value = serde_json::to_value(&parsed).expect("failed to serialize OpenAPI document");
        let info = value
            .get("info")
            .expect("OpenAPI document must contain info");
        let title = info
            .get("title")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| panic!("{}: info.title is required", path.display()))
            .to_string();
        let description = info
            .get("description")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| panic!("{}: info.description is required", path.display()))
            .to_string();
        let slug = info
            .get("x-bionic-slug")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string());
        let source_url = info
            .get("x-bionic-source-url")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| panic!("{}: info.x-bionic-source-url is required", path.display()))
            .to_string();
        let source_logo_url = info
            .get("x-logo")
            .and_then(|logo| logo.get("url"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string();
        if !slug
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
        {
            panic!(
                "{}: info.x-bionic-slug must use only letters, numbers, and hyphens",
                path.display()
            );
        }
        let has_embedded_logo = source_logo_url.starts_with("data:image/svg+xml;base64,");
        let logo_url = if has_embedded_logo {
            logo_url_for_slug(&slug)
        } else {
            String::new()
        };
        integrations.push(Integration {
            slug,
            title,
            description,
            logo_url,
            logo_data_uri: if has_embedded_logo {
                source_logo_url
            } else {
                String::new()
            },
            source_url,
            filename,
        });
    }

    let catalog =
        serde_json::to_string(&integrations).expect("failed to encode integration catalogue");
    fs::write(output_dir.join("integrations-catalog.json"), catalog)
        .expect("failed to write generated integrations catalogue");
    fs::write(output_dir.join("integrations-curated-doc.md"), docs)
        .expect("failed to write generated integrations documentation");
}

fn logo_url_for_slug(slug: &str) -> String {
    format!("/integrations/logos/{slug}.svg")
}

fn escape_html(source: &str) -> String {
    source
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let content_dir =
        manifest_dir.join("content/architect-course/enterprise-evals/dashboard-builder");
    let package_dir = content_dir.join("package");
    let page =
        fs::read_to_string(content_dir.join("index.md")).expect("failed to read dashboard page");
    let skill = fs::read_to_string(package_dir.join("SKILL.md"))
        .expect("failed to read dashboard skill source");
    let renderer = fs::read_to_string(package_dir.join("bin/render_dashboard.py"))
        .expect("failed to read dashboard renderer source");

    let page = page
        .replace(
            "<!-- DASHBOARD_SKILL_SOURCE -->",
            &format!(
                "<details>\n<summary>View SKILL.md</summary>\n\n<pre><code class=\"language-markdown\">{}\n</code></pre>\n</details>",
                escape_html(&skill)
            ),
        )
        .replace(
            "<!-- DASHBOARD_RENDERER_SOURCE -->",
            &format!(
                "<details>\n<summary>View render_dashboard.py</summary>\n\n<pre><code class=\"language-python\">{}\n</code></pre>\n</details>",
                escape_html(&renderer)
            ),
        );

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("dashboard-builder-page.md");
    fs::write(output, page).expect("failed to write generated dashboard page");

    println!(
        "cargo:rerun-if-changed={}",
        content_dir.join("index.md").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        package_dir.join("SKILL.md").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        package_dir.join("bin/render_dashboard.py").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir
            .join("content/docs/integrations/specs")
            .display()
    );
    generate_integrations(&manifest_dir);
}
