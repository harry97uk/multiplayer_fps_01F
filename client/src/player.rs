use bevy::prelude::*;
use renet::RenetClient;
use store::player::PlayerDirection;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub z: f32,
    pub direction: PlayerDirection,
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut client: ResMut<RenetClient>
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let mut cam_transform = cam_q.single_mut();

    let mut direction = Vec3::ZERO;

    //all of this should be in a game event that is sent to the server
    if keyboard_input.just_released(KeyCode::Up) {
        let bevy_vec3 = Vec3::new(
            player.direction.value().x,
            player.direction.value().y,
            player.direction.value().z
        );
        direction += bevy_vec3;
    }

    player.x = player.x + direction.x;
    player.z = player.z + direction.z;

    player_transform.translation += direction;

    cam_transform.translation += direction;
}

pub fn player_rotation(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut client: ResMut<RenetClient>
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let mut cam_transform = cam_q.single_mut();

    let mut angle_in_degrees = 0.0;
    let mut direction = player.direction.clone();

    //all of this should be in a game event that is sent to the server
    if keyboard_input.just_released(KeyCode::Left) {
        angle_in_degrees = 90.0;
        direction = match player.direction {
            PlayerDirection::North => PlayerDirection::West,
            PlayerDirection::East => PlayerDirection::North,
            PlayerDirection::South => PlayerDirection::East,
            PlayerDirection::West => PlayerDirection::South,
        };
    }

    if keyboard_input.just_released(KeyCode::Right) {
        angle_in_degrees = -90.0;
        direction = match player.direction {
            PlayerDirection::North => PlayerDirection::East,
            PlayerDirection::East => PlayerDirection::South,
            PlayerDirection::South => PlayerDirection::West,
            PlayerDirection::West => PlayerDirection::North,
        };
    }

    let angle_in_radians = (angle_in_degrees * PI) / 180.0;

    player.direction = direction;

    player_transform.rotate_axis(Vec3::Y, angle_in_radians);

    cam_transform.rotate_axis(Vec3::Y, angle_in_radians);
}
