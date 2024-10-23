use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn generate_static_files_code(out_dir: &Path, asset_dirs: &[PathBuf]) -> std::io::Result<()> {
    let mut output = String::new();
    let mut static_file_names = Vec::new();

    // Add the StaticFile struct definition to the output
    output.push_str(
        r#"
    pub mod statics {
        pub struct StaticFile {
            pub file_name: &'static str,
            pub name: &'static str,
            pub mime: mime::Mime,
        }
    "#,
    );

    // Process each asset directory provided
    for asset_dir in asset_dirs {
        process_directory(asset_dir, &mut output, &mut static_file_names)?;
    }

    output.push_str(
        r#"#[allow(dead_code)]
        impl StaticFile {
            /// Get a single `StaticFile` by name, if it exists.
            #[must_use]
            pub fn get(name: &str) -> Option<&'static Self> {
                if let Some(pos) = STATICS.iter().position(|&s| name == s.name) {
                    Some(STATICS[pos])
                } else {None}
            }
        }
    "#,
    );

    let statics_array = static_file_names
        .iter()
        .map(|name| format!("&{}", name))
        .collect::<Vec<_>>()
        .join(", ");

    output.push_str(&format!(
        "pub static STATICS: &[&StaticFile] = &[{}];",
        statics_array
    ));

    output.push('}');

    // Write the generated code to the output file
    let out_file_path = out_dir.join("static_files.rs");
    let mut out_file = File::create(out_file_path)?;
    out_file.write_all(output.as_bytes())?;

    Ok(())
}

fn process_directory(
    dir: &Path,
    output: &mut String,
    static_file_names: &mut Vec<String>,
) -> std::io::Result<()> {
    // Walk through the directory recursively
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively process subdirectories
            process_directory(&path, output, static_file_names)?;
        } else if path.is_file() {
            // Get the full path using canonicalize
            let full_path = fs::canonicalize(&path)?;
            let file_name = full_path.to_str().unwrap();

            // Generate the hash for cache busting using MD5 and hex encoding
            let hash = calculate_hash(&path)?;

            // Generate a static-friendly variable name (e.g., assistant_svg)
            let var_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(['/', '.', '-'], "_");

            // Construct the new hashed filename
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let extension = path.extension().unwrap().to_str().unwrap();
            let hashed_name = format!("{file_stem}-{hash}.{extension}");

            // Generate Rust code for the static file
            output.push_str(&format!(
                r#"
                /// From "{file_name}"
                #[allow(non_upper_case_globals)]
                pub static {var_name}: StaticFile = StaticFile {{
                    file_name: "{file_name}",
                    name: "/static/{hashed_name}",
                    mime: mime::{},
                }};
                "#,
                mime_type_from_extension(extension),
            ));

            // Collect the variable name for the STATICS array
            static_file_names.push(var_name);
        }
    }

    Ok(())
}

// Helper function to generate a hash for a file's contents using MD5 and hex encoding
fn calculate_hash(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    // Read the entire file into memory
    file.read_to_end(&mut buffer)?;

    // Compute the MD5 hash and return it as a hex string
    let hash = md5::compute(&buffer);
    Ok(format!("{:x}", hash))
}

// Helper function to map file extensions to MIME types
fn mime_type_from_extension(extension: &str) -> &'static str {
    match extension {
        "svg" => "IMAGE_SVG",
        "png" => "IMAGE_PNG",
        "jpg" | "jpeg" => "IMAGE_JPEG",
        "css" => "TEXT_CSS",
        "js" => "APPLICATION_JAVASCRIPT",
        _ => "APPLICATION_OCTET_STREAM",
    }
}
