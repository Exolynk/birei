use std::error::Error;
use std::fs;

use super::BuildPaths;

pub fn copy_assets(paths: &BuildPaths) -> Result<(), Box<dyn Error>> {
    // Place the Lucide font next to `dist/birei.css` so consumers can serve both files with
    // the relative font URL preserved.
    fs::create_dir_all(&paths.output_dir)?;
    fs::copy(&paths.lucide_font, &paths.dist_lucide_font)?;

    Ok(())
}
