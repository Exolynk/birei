use std::error::Error;
use std::path::PathBuf;

mod assets;
mod css;
mod icons;

const LUCIDE_DIR: &str = "deps/lucide0-577-0";
const LUCIDE_SCSS_FILE: &str = "lucide.scss";
const LUCIDE_FONT_FILE: &str = "lucide.woff2";
const ICON_NAMES_RS_FILE: &str = "src/icon/icn_names.rs";

pub fn run() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let paths = BuildPaths::new(manifest_dir);

    css::track_inputs(&paths)?;
    icons::generate_icon_names(&paths)?;
    css::compile_bundle(&paths)?;
    assets::copy_assets(&paths)?;

    Ok(())
}

pub struct BuildPaths {
    pub manifest_dir: PathBuf,
    pub styles_dir: PathBuf,
    pub source_dir: PathBuf,
    pub entrypoint: PathBuf,
    pub output_dir: PathBuf,
    pub output_file: PathBuf,
    pub lucide_scss: PathBuf,
    pub lucide_font: PathBuf,
    pub icon_names_rs: PathBuf,
    pub dist_lucide_font: PathBuf,
}

impl BuildPaths {
    fn new(manifest_dir: PathBuf) -> Self {
        let styles_dir = manifest_dir.join("styles");
        let source_dir = manifest_dir.join("src");
        let entrypoint = styles_dir.join("index.scss");
        let output_dir = manifest_dir.join("dist");
        let output_file = output_dir.join("birei.css");
        let lucide_dir = manifest_dir.join(LUCIDE_DIR);
        let lucide_scss = lucide_dir.join(LUCIDE_SCSS_FILE);
        let lucide_font = lucide_dir.join(LUCIDE_FONT_FILE);
        let icon_names_rs = manifest_dir.join(ICON_NAMES_RS_FILE);
        let dist_lucide_font = output_dir.join(LUCIDE_FONT_FILE);

        Self {
            manifest_dir,
            styles_dir,
            source_dir,
            entrypoint,
            output_dir,
            output_file,
            lucide_scss,
            lucide_font,
            icon_names_rs,
            dist_lucide_font,
        }
    }
}
