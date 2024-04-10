use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
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
    mut query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
) {
    for (mut transform, projection) in query.iter_mut() {
        let speed = projection.scale * 4.0;
        let mut translation = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            translation.y += speed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            translation.y -= speed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            translation.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            translation.x += speed;
        }
        transform.translation += translation;
    }
}

pub fn scale_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    for ev in scroll_evr.read() {
        for mut projection in query.iter_mut() {
            projection.scale *= 1.0 + ev.y * 0.1;
        }
    }
}
