use cache_busters::generate_static_files_code;
use std::{env, path::PathBuf};

// In your `build.rs`, call the function
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Example of multiple asset directories
    let asset_dirs = vec![
        PathBuf::from("./dist"), // Adjust your asset directories here
        PathBuf::from("./assets"),
    ];

    generate_static_files_code(&out_dir, &asset_dirs).unwrap();
}
