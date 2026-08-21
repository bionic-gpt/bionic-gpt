use std::env;
use std::fs;
use std::path::PathBuf;

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
}
