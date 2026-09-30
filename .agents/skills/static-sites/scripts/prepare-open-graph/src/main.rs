use std::{env, fs, path::Path, process};

use image::{codecs::jpeg::JpegEncoder, imageops::FilterType};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 630;
const MAX_BYTES: usize = 490_000;
const MAX_QUALITY: u8 = 85;
const MIN_QUALITY: u8 = 40;
const QUALITY_STEP: usize = 5;

fn fail(message: impl std::fmt::Display) -> ! {
    eprintln!("prepare-open-graph: {message}");
    process::exit(1);
}

fn main() {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .unwrap_or_else(|| fail("missing input image path"));
    let output = args
        .next()
        .unwrap_or_else(|| fail("missing output JPEG path"));
    if args.next().is_some() {
        fail("usage: prepare-open-graph <input-image> <output.jpg>");
    }

    let extension = Path::new(&output)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if !extension.eq_ignore_ascii_case("jpg") && !extension.eq_ignore_ascii_case("jpeg") {
        fail("output path must end in .jpg or .jpeg");
    }

    let source = image::open(&input)
        .unwrap_or_else(|error| fail(format!("could not decode {input}: {error}")));
    let resized = source
        .resize_to_fill(WIDTH, HEIGHT, FilterType::Lanczos3)
        .to_rgb8();

    let mut smallest_size = usize::MAX;
    for quality in (MIN_QUALITY..=MAX_QUALITY).rev().step_by(QUALITY_STEP) {
        let mut bytes = Vec::new();
        JpegEncoder::new_with_quality(&mut bytes, quality)
            .encode_image(&resized)
            .unwrap_or_else(|error| fail(format!("could not encode JPEG: {error}")));
        smallest_size = bytes.len();

        if bytes.len() < MAX_BYTES {
            if let Some(parent) = Path::new(&output).parent() {
                fs::create_dir_all(parent).unwrap_or_else(|error| {
                    fail(format!("could not create {}: {error}", parent.display()))
                });
            }
            fs::write(&output, &bytes)
                .unwrap_or_else(|error| fail(format!("could not write {output}: {error}")));
            println!(
                "wrote {output}: {WIDTH}x{HEIGHT}, quality {quality}, {} bytes",
                bytes.len()
            );
            return;
        }
    }

    fail(format!(
        "could not meet the {MAX_BYTES}-byte limit; quality {MIN_QUALITY} produced {smallest_size} bytes"
    ));
}
