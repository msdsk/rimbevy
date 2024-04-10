use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use bevy::prelude::*;

use noise::{utils::NoiseMap, Exponent, NoiseFn, Perlin};

const tile_size: f32 = 64.0;
const isometry_ratio: f32 = 0.5;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            .add_systems(Update, render_tile);
    }
}

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    position: Position,
    sprite: SpriteBundle,
}

fn generate_noise() -> Perlin {
    Perlin::new(0)
}

fn render_tile(mut query: Query<(&Position, &mut Transform), Added<Tile>>) {
    for (position, mut transform) in query.iter_mut() {
        let (x, y, z) = (position.x, position.y, position.z);
        transform.translation = Vec3::new(
            (x - y) as f32 * (tile_size * 0.5),
            (x + y) as f32 * (tile_size * 0.5) * isometry_ratio,
            -(y + x) as f32,
        );
    }
}

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("sprites/terrain/default.png");
    let width = MAP_WIDTH;
    let height = MAP_HEIGHT;

    let noise = generate_noise();

    for x in 0..(width) {
        for y in 0..(height) {
            let z = noise.get([x as f64 + 0.1, y as f64 + 0.3]);
            commands.spawn(TileBundle {
                tile: Tile,
                position: Position {
                    x,
                    y,
                    z: z.floor() as i32,
                },
                sprite: SpriteBundle {
                    texture: texture_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });
        }
    }
}
