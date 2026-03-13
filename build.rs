use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let styles_dir = manifest_dir.join("styles");
    let source_dir = manifest_dir.join("src");
    let entrypoint = styles_dir.join("index.scss");
    let output_dir = manifest_dir.join("dist");
    let output_file = output_dir.join("birei.css");

    println!("cargo:rerun-if-changed={}", entrypoint.display());
    println!("cargo:rerun-if-changed={}", output_file.display());
    track_scss_files(&source_dir)?;
    track_scss_files(&styles_dir)?;

    let css = grass::from_path(
        &entrypoint,
        &grass::Options::default().load_path(&manifest_dir),
    )?;

    fs::create_dir_all(&output_dir)?;
    fs::write(output_file, css)?;

    Ok(())
}

fn track_scss_files(dir: &Path) -> Result<(), Box<dyn Error>> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            track_scss_files(&path)?;
            continue;
        }

        if path.extension().is_some_and(|ext| ext == "scss") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    Ok(())
}
