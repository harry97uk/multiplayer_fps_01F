use bevy::prelude::*;
use renet::RenetClient;
use store::{ player::PlayerDirection, game::{ GameState, GameEvent } };

use crate::helper::convert_degrees_to_radians;

use super::{ AppState, components::Materials };

pub struct PlayerPlugin;

#[derive(Debug, PartialEq, Clone, Component)]
pub struct BevyPlayer {
    pub id: u64,
    pub name: String,
    pub position: Vec3,
    pub direction: PlayerDirection,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(insert_debug_info)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_client_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(send_player_movement)
                    .with_system(update_players)
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_player));
    }
}

fn insert_debug_info() {
    println!("Debug Info: Inserting BevyPlayer component");
}

fn cleanup_player(mut commands: Commands) {
    commands.remove_resource::<BevyPlayer>()
}

fn spawn_client_player(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: Res<Materials>,
    game_state: Res<GameState>,
    client: ResMut<RenetClient>
) {
    println!("spawning client player");

    let player_mesh = mesh_assets.add(
        Mesh::from(shape::Icosphere { radius: 0.2, subdivisions: 5 })
    );

    commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: player_mesh,
            material: materials.player_material.clone(),
            transform: Transform::from_xyz(1.0, 0.2, 1.0),
            ..Default::default()
        })
        .insert(BevyPlayer {
            id: client.client_id(),
            name: "frfrf".to_string(),
            position: Vec3 { x: 1.0, y: 0.2, z: 1.0 },
            direction: PlayerDirection::East,
        });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 0.5, 1.0).looking_at(
            Vec3 { x: 2.0, y: 0.5, z: 1.0 },
            Vec3::Y
        ),
        ..Default::default()
    });

    for player in game_state.players.iter() {
        if *player.0 != client.client_id() {
            spawn_player(&mut commands, &mut mesh_assets, &mut materials, &game_state, *player.0);
        }
    }
}

fn spawn_player(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    materials: &mut Res<Materials>,
    game_state: &Res<GameState>,
    player_id: u64
) {
    println!("spawning player");
    let player_mesh = mesh_assets.add(
        Mesh::from(shape::Icosphere { radius: 0.2, subdivisions: 5 })
    );

    for player in game_state.players.iter() {
        if *player.0 == player_id {
            commands
                .spawn()
                .insert_bundle(PbrBundle {
                    mesh: player_mesh.clone(),
                    material: materials.player_material.clone(),
                    transform: Transform::from_xyz(player.1.x, player.1.y, player.1.z),
                    ..Default::default()
                })
                .insert(BevyPlayer {
                    direction: PlayerDirection::East,
                    id: player_id,
                    name: player.1.name.to_string(),
                    position: Vec3 { x: player.1.x, y: player.1.y, z: player.1.z },
                });
        }
    }
}

pub fn send_player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut BevyPlayer>,
    mut client: ResMut<RenetClient>
) {
    for player in player_query.iter_mut() {
        if player.id != client.client_id() {
            continue;
        }

        if keyboard_input.just_released(KeyCode::Up) {
            let mut direction = Vec3::ZERO;
            let bevy_vec3 = Vec3::new(
                player.direction.value().x,
                player.direction.value().y,
                player.direction.value().z
            );
            direction += bevy_vec3;

            let event = GameEvent::PlayerMoved {
                player_id: client.client_id(),
                new_position: (player.position.x + direction.x, player.position.z + direction.z),
                new_direction: (player.direction, 0.0),
            };
            client.send_message(0, bincode::serialize(&event).unwrap());
        }
        if keyboard_input.just_released(KeyCode::Down) {
            let mut direction = Vec3::ZERO;
            let bevy_vec3 = Vec3::new(
                player.direction.value().x,
                player.direction.value().y,
                player.direction.value().z
            );
            direction -= bevy_vec3;

            let event = GameEvent::PlayerMoved {
                player_id: client.client_id(),
                new_position: (player.position.x + direction.x, player.position.z + direction.z),
                new_direction: (player.direction, 0.0),
            };
            client.send_message(0, bincode::serialize(&event).unwrap());
        }
        if keyboard_input.just_released(KeyCode::Left) {
            let angle_in_degrees = 90.0;
            let direction = match player.direction {
                PlayerDirection::North => PlayerDirection::West,
                PlayerDirection::East => PlayerDirection::North,
                PlayerDirection::South => PlayerDirection::East,
                PlayerDirection::West => PlayerDirection::South,
            };

            let angle_in_radians = convert_degrees_to_radians(angle_in_degrees);

            let event = GameEvent::PlayerMoved {
                player_id: client.client_id(),
                new_position: (player.position.x, player.position.z),
                new_direction: (direction, angle_in_radians),
            };
            client.send_message(0, bincode::serialize(&event).unwrap());
        }
        if keyboard_input.just_released(KeyCode::Right) {
            let angle_in_degrees = -90.0;
            let direction = match player.direction {
                PlayerDirection::North => PlayerDirection::East,
                PlayerDirection::East => PlayerDirection::South,
                PlayerDirection::South => PlayerDirection::West,
                PlayerDirection::West => PlayerDirection::North,
            };

            let angle_in_radians = convert_degrees_to_radians(angle_in_degrees);

            let event = GameEvent::PlayerMoved {
                player_id: client.client_id(),
                new_position: (player.position.x, player.position.z),
                new_direction: (direction, angle_in_radians),
            };
            client.send_message(0, bincode::serialize(&event).unwrap());
        }
    }
}

pub fn update_players(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: Res<Materials>,
    game_state: Res<GameState>,
    mut player_query: Query<(&mut BevyPlayer, &mut Transform)>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<BevyPlayer>)>,
    mut game_events: EventReader<GameEvent>,
    client: ResMut<RenetClient>
) {
    for event in game_events.iter() {
        match event {
            GameEvent::PlayerJoined { player_id, .. } => {
                if *player_id != client.client_id() {
                    spawn_player(
                        &mut commands,
                        &mut mesh_assets,
                        &mut materials,
                        &game_state,
                        *player_id
                    );
                }
            }
            GameEvent::PlayerMoved { player_id, new_position, new_direction } => {
                for (mut player, mut transform) in player_query.iter_mut() {
                    if player.id == *player_id {
                        player.position.x = new_position.0;
                        player.position.z = new_position.1;
                        player.direction = new_direction.0;

                        transform.translation = Vec3::new(new_position.0, 0.2, new_position.1);
                        transform.rotate_axis(Vec3::Y, new_direction.1);

                        if *player_id == client.client_id() {
                            let mut cam_transform = cam_q.single_mut();

                            cam_transform.translation = Vec3::new(
                                new_position.0,
                                0.5,
                                new_position.1
                            );
                            cam_transform.rotate_axis(Vec3::Y, new_direction.1);
                        }
                    }
                }
            }

            GameEvent::PlayerDisconnected { player_id } => {
                println!("{} has disconnected", player_id);
                for player in game_state.players.iter() {
                    if *player.0 == *player_id {
                        //remove player
                    }
                }
            }
        }
    }
}
