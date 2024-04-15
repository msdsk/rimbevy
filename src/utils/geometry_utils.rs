use bevy::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH, Z_HEIGHT};

// pub fn world_to_map(world_x: f32, world_y: f32, z: Option<i32>) {}

pub fn map_to_world(point: IVec3) -> Vec2 {
    let (x, y, z) = point.into();
    Vec2::new(
        (x - y) as f32 * (TILE_WIDTH * 0.5),
        (x + y) as f32 * (TILE_HEIGHT * 0.5) + z as f32 * Z_HEIGHT * 0.5,
    )
}
