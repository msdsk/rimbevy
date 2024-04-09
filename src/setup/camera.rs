use bevy::input::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));
}

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for (mut transform) in query.iter_mut() {
        let mut translation = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            translation.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            translation.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            translation.x += 1.0;
        }
        transform.translation += translation * 4.0;
    }
}
