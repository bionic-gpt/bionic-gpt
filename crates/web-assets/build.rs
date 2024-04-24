use ructe::{Result, Ructe};
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Compile our templates
    ructe()?;

    // Ruct puts all assets into your binary. We don't want to do that, So we update the generated
    // ructe code.
    // See https://github.com/kaj/ructe/issues/112
    // pub content: &'static [u8],
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path_buf = PathBuf::from(&out_dir.into_string().unwrap());
    let file_path = path_buf.join("templates/statics.rs");

    // s/pub content: \&\x27static \[u8\]/pub file_name: \&\x27static str/g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/pub content: \&\x27static \[u8\]/pub file_name: \&\x27static str/g")
        .arg(file_path.clone())
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    // s/content: include_bytes!(/file_name: /g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/content: include_bytes!(/file_name: /g")
        .arg(file_path.clone())
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    // Make the name have the route included i.e. /static
    // s/content: include_bytes!(/file_name: /g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/[^_]name: \x22/ name: \x22\/static\//g")
        .arg(file_path.clone())
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    // s/),/,/g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/),/,/g")
        .arg(file_path)
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    Ok(())
}

fn ructe() -> Result<()> {
    // Compile our templates
    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();

    statics.add_files("./images").unwrap();
    statics.add_files("./images/layout").unwrap();
    statics.add_files("./images/sidebar").unwrap();
    statics.add_files("./dist").unwrap();
    ructe.compile_templates("./dist").unwrap();

    Ok(())
}
