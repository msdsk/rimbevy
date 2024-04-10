use crate::constants::{ISOMETRY_RATIO, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE, Z_LEVELS};
use bevy::{prelude::*};

use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder}, Fbm, MultiFractal, Perlin,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            .add_systems(Update, render_terrain);
    }
}

#[derive(Component)]
pub struct SpriteTransform(Vec2);

#[derive(Component)]
pub struct Layer(i8);

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
    layer: Layer,
    sprite: SpriteBundle,
    sprite_transform: SpriteTransform,
}

fn create_tile(asset_server: Handle<Image>, x: i32, y: i32, z: i32) -> TileBundle {
    TileBundle {
        tile: Tile,
        position: Position { x, y, z },
        layer: Layer(1),
        sprite: SpriteBundle {
            texture: asset_server,
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        },
        sprite_transform: SpriteTransform(Vec2::new(0.0, 0.0)),
    }
}

#[derive(Component)]
pub struct Stone;

#[derive(Component)]
pub struct Item;

#[derive(Bundle)]
pub struct StoneBundle {
    item: Item,
    stone: Stone,
    position: Position,
    sprite: SpriteBundle,
    layer: Layer,
    sprite_transform: SpriteTransform,
}

fn create_stone(asset_server: Handle<Image>, x: i32, y: i32, z: i32) -> StoneBundle {
    StoneBundle {
        item: Item,
        stone: Stone,
        position: Position { x, y, z },
        layer: Layer(0),
        sprite: SpriteBundle {
            texture: asset_server,
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        },
        sprite_transform: SpriteTransform(Vec2::new(0.0, -0.257)),
    }
}

fn generate_noise() -> NoiseMap {
    let fbm = Fbm::<Perlin>::default()
        .set_octaves(3)
        .set_persistence(0.2)
        .set_frequency(0.1);
    PlaneMapBuilder::new(&fbm)
        .set_size(MAP_WIDTH as usize, MAP_HEIGHT as usize)
        .set_x_bounds(0.0, Z_LEVELS as f64)
        .set_y_bounds(0.0, Z_LEVELS as f64)
        .build()
}

fn render_terrain(
    mut query: Query<(&Position, &Layer, &SpriteTransform, &mut Transform), Added<Position>>,
) {
    for (position, layer, sprite_transform, mut transform) in query.iter_mut() {
        let (x, y, z) = (position.x, position.y, position.z);
        transform.translation = Vec3::new(
            (x - y) as f32 * (TILE_SIZE * 0.5) + sprite_transform.0.x * TILE_SIZE,
            (x + y) as f32 * (TILE_SIZE * 0.5) * ISOMETRY_RATIO
                + z as f32 * TILE_SIZE * 0.5
                + sprite_transform.0.y * TILE_SIZE,
            -(y + x) as f32 + z as f32 * 0.1 + layer.0 as f32 * 0.01,
        );
    }
}

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ground_texture_handle = asset_server.load("sprites/terrain/default.png");
    let stone_texture_handle = asset_server.load("sprites/terrain/stone.png");
    let width = MAP_WIDTH;
    let height = MAP_HEIGHT;

    let noise = generate_noise();

    for x in 0..(width) {
        for y in 0..(height) {
            let z: i32 = (0.1 * Z_LEVELS as f64
                + (noise.get_value(x as usize, y as usize) + 1.0) * 0.5 * Z_LEVELS as f64)
                as i32;

            commands.spawn(create_tile(ground_texture_handle.clone(), x, y, z));

            for z_level in 0..((z + 1) as i32) {
                commands.spawn(create_stone(stone_texture_handle.clone(), x, y, z_level));
            }
        }
    }
}
