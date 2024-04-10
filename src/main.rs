use bevy::prelude::*;

mod constants;
mod setup;
use setup::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, move_camera)
        .add_systems(Update, scale_camera)
        .run();
}
