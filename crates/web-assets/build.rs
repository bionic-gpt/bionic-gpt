use cache_busters::generate_static_files_code;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Install dependencies and build the front-end assets.
    Command::new("npm")
        .arg("ci")
        .status()
        .expect("failed to run `npm ci`");

    // Build the javascript and CSS bundles. This runs the `release` script which
    // invokes tailwind-extra and Parcel to create files under `dist/`.
    Command::new("npm")
        .args(["run", "release"])
        .status()
        .expect("failed to run `npm run release`");

    let static_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let asset_dirs = vec![PathBuf::from("./dist"), PathBuf::from("./images")];

    generate_static_files_code(&static_out_dir, &asset_dirs).unwrap();
}
