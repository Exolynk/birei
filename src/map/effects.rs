use leptos::prelude::*;

use super::geo::MapCoordinate;

pub(crate) fn sync_center_prop(
    center: MaybeProp<MapCoordinate>,
    viewport_center: RwSignal<MapCoordinate>,
) {
    Effect::new(move |_| {
        if let Some(next_center) = center.get() {
            viewport_center.set(next_center);
        }
    });
}
