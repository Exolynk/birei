use std::f64::consts::PI;

const TILE_SIZE: f64 = 256.0;
const MIN_LATITUDE: f64 = -85.051_128_78;
const MAX_LATITUDE: f64 = 85.051_128_78;

pub(crate) const DEFAULT_CENTER: MapCoordinate = MapCoordinate::new(47.3769, 8.5417);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapCoordinate {
    pub lat: f64,
    pub lng: f64,
}

impl MapCoordinate {
    pub const fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct WorldPoint {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MapTile {
    pub(crate) key: String,
    pub(crate) url: String,
    pub(crate) left: f64,
    pub(crate) top: f64,
}

pub(crate) fn compute_visible_tiles(
    center: MapCoordinate,
    zoom: u8,
    size: (f64, f64),
) -> Vec<MapTile> {
    let (width, height) = size;
    if width <= 0.0 || height <= 0.0 {
        return Vec::new();
    }

    let world_size = world_size(zoom);
    let center_world = project(center, zoom);
    let top_left = WorldPoint {
        x: center_world.x - (width / 2.0),
        y: center_world.y - (height / 2.0),
    };
    let bottom_right = WorldPoint {
        x: center_world.x + (width / 2.0),
        y: center_world.y + (height / 2.0),
    };
    let tile_count = 2_i32.pow(u32::from(zoom));
    let min_x = (top_left.x / TILE_SIZE).floor() as i32;
    let max_x = (bottom_right.x / TILE_SIZE).floor() as i32;
    let min_y = (top_left.y / TILE_SIZE).floor().max(0.0) as i32;
    let max_y = (bottom_right.y / TILE_SIZE)
        .floor()
        .min(((world_size / TILE_SIZE) - 1.0).max(0.0)) as i32;
    let mut tiles = Vec::new();

    for tile_y in min_y..=max_y {
        for tile_x in min_x..=max_x {
            let wrapped_x = tile_x.rem_euclid(tile_count);
            let left = (f64::from(tile_x) * TILE_SIZE) - top_left.x;
            let top = (f64::from(tile_y) * TILE_SIZE) - top_left.y;
            tiles.push(MapTile {
                key: format!("{zoom}-{tile_x}-{tile_y}"),
                url: format!("https://tile.openstreetmap.org/{zoom}/{wrapped_x}/{tile_y}.png"),
                left,
                top,
            });
        }
    }

    tiles
}

pub(crate) fn marker_style(
    position: MapCoordinate,
    viewport_center: MapCoordinate,
    zoom: u8,
    map_size: (f64, f64),
) -> String {
    let (width, height) = map_size;
    let point = project(position, zoom);
    let center = project(viewport_center, zoom);
    let left = point.x - center.x + (width / 2.0);
    let top = point.y - center.y + (height / 2.0);

    format!("left: {left:.3}px; top: {top:.3}px;")
}

pub(crate) fn project(coordinate: MapCoordinate, zoom: u8) -> WorldPoint {
    let scale = world_size(zoom);
    let lat = coordinate.lat.clamp(MIN_LATITUDE, MAX_LATITUDE);
    let lng = wrap_longitude(coordinate.lng);
    let lat_rad = lat.to_radians();
    let x = ((lng + 180.0) / 360.0) * scale;
    let y = (1.0 - ((lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / PI)) * 0.5 * scale;

    WorldPoint { x, y }
}

pub(crate) fn unproject(point: WorldPoint, zoom: u8) -> MapCoordinate {
    let world_size = world_size(zoom);
    let wrapped_x = point.x.rem_euclid(world_size);
    let clamped_y = point.y.clamp(0.0, world_size);
    let lng = (wrapped_x / world_size) * 360.0 - 180.0;
    let mercator = PI * (1.0 - (2.0 * clamped_y / world_size));
    let lat = mercator
        .sinh()
        .atan()
        .to_degrees()
        .clamp(MIN_LATITUDE, MAX_LATITUDE);

    MapCoordinate { lat, lng }
}

fn world_size(zoom: u8) -> f64 {
    TILE_SIZE * f64::from(2_u32.pow(u32::from(zoom)))
}

fn wrap_longitude(lng: f64) -> f64 {
    ((lng + 180.0).rem_euclid(360.0)) - 180.0
}
