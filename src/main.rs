use bevy::prelude::*;

mod setup;
use setup::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_terrain)
        .add_systems(Update, move_camera)
        .run();
}
