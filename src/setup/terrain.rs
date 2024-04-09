use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    x: i32,
    y: i32,
}

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("sprites/terrain/default.png");
    let width = 100.0;
    let height = 100.0;
    let tile_size = 64.0;
    let isometry_ratio: f32 = 0.5;

    for x in 0..(width as i32) {
        for y in 0..(height as i32) {
            commands.spawn((
                Tile { x, y },
                SpriteBundle {
                    texture: texture_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x - y) as f32 * (tile_size * 0.5),
                        (x + y) as f32 * (tile_size * 0.5) * isometry_ratio,
                        -(y + x) as f32,
                    ),
                    ..Default::default()
                },
            ));
        }
    }
}
