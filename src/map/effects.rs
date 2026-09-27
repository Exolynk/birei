use leptos::prelude::*;

use super::geo::MapCoordinate;

/// Keeps the internally managed viewport center aligned with an optional
/// controlled `center` prop from the parent.
pub(crate) fn sync_center_prop(
    center: MaybeProp<MapCoordinate>,
    viewport_center: RwSignal<MapCoordinate>,
) {
    Effect::new(move |_| {
        if let Some(next_center) = center.try_get().unwrap_or_default() {
            viewport_center.set(next_center);
        }
    });
}
