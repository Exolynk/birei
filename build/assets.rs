use std::error::Error;
use std::fs;

use super::{copy_if_changed, BuildPaths};

pub fn copy_assets(paths: &BuildPaths) -> Result<(), Box<dyn Error>> {
    // Place bundled font assets next to `dist/birei.css` so relative URLs remain valid.
    fs::create_dir_all(&paths.output_dir)?;
    copy_if_changed(&paths.lucide_font, &paths.dist_lucide_font)?;
    copy_if_changed(
        &paths.instrument_sans_font,
        &paths.dist_instrument_sans_font,
    )?;
    copy_if_changed(
        &paths.instrument_sans_italic_font,
        &paths.dist_instrument_sans_italic_font,
    )?;

    Ok(())
}
