use std::error::Error;
use std::fs;
use std::path::Path;

use super::{write_if_changed, BuildPaths};

pub fn track_inputs(paths: &BuildPaths) -> Result<(), Box<dyn Error>> {
    // Rebuild whenever the stylesheet entrypoint, bundled Lucide assets, or any nested
    // component SCSS file changes so `dist/birei.css` stays in sync with the crate sources.
    println!("cargo:rerun-if-changed={}", paths.entrypoint.display());
    println!("cargo:rerun-if-changed={}", paths.lucide_scss.display());
    println!("cargo:rerun-if-changed={}", paths.lucide_font.display());
    println!(
        "cargo:rerun-if-changed={}",
        paths.instrument_sans_font.display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        paths.instrument_sans_italic_font.display()
    );

    track_scss_files(&paths.source_dir)?;
    track_scss_files(&paths.styles_dir)?;

    Ok(())
}

pub fn compile_bundle(paths: &BuildPaths) -> Result<(), Box<dyn Error>> {
    // Compile the library SCSS pipeline into a distributable CSS file. `manifest_dir` is
    // added as a Sass load path so imports can reference project-local files like `deps/...`.
    let css = grass::from_path(
        &paths.entrypoint,
        &grass::Options::default().load_path(&paths.manifest_dir),
    )?;

    fs::create_dir_all(&paths.output_dir)?;
    write_if_changed(&paths.output_file, css.as_bytes())?;

    Ok(())
}

fn track_scss_files(dir: &Path) -> Result<(), Box<dyn Error>> {
    if !dir.exists() {
        return Ok(());
    }

    // Walk the tree so Cargo reruns the build script when any component-level SCSS file is
    // edited, even if that file is only imported indirectly through the main stylesheet.
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
