use std::env;
use std::fs;
use std::path::PathBuf;

use jpg2ascii::{convert_path_to_ascii, Config};

fn main() -> anyhow::Result<()> {
    // Optional arg: path to image. If not provided, search ./examples for the first image.
    let args: Vec<String> = env::args().skip(1).collect();
    let img_path = if let Some(p) = args.get(0) {
        PathBuf::from(p)
    } else if let Some(p) = find_example_image()? {
        println!("[info] Using example image: {}", p.display());
        p
    } else {
        println!(
            "No image argument provided and no image found under ./examples.\n  - Run: cargo run --example basic -- <image_path>\n  - Or place a .jpg/.jpeg/.png/.gif in ./examples and rerun"
        );
        std::process::exit(1);
    };

    let cfg = Config { width: Some(80), ..Default::default() };
    let ascii = convert_path_to_ascii(&img_path, &cfg)?;
    println!("{}", ascii);
    Ok(())
}

fn find_example_image() -> anyhow::Result<Option<PathBuf>> {
    let dir = PathBuf::from("examples");
    if !dir.exists() { return Ok(None); }
    let exts = ["jpg", "jpeg", "png", "gif"]; 
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            if exts.iter().any(|x| x.eq_ignore_ascii_case(ext)) {
                return Ok(Some(p));
            }
        }
    }
    Ok(None)
}
