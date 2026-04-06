// Map rendering is split into geometry, interaction, effects, and view helpers
// so the top-level component stays readable despite browser-event complexity.
mod effects;
mod geo;
mod interaction;
mod map_viewer;
mod view;

pub use geo::MapCoordinate;
pub use map_viewer::MapViewer;
