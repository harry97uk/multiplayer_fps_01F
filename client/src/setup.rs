use bevy::prelude::*;
use store::{ game::GameState, player::PlayerDirection };

use crate::player::Player;

pub fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_state: Res<GameState>
) {
    let player_mesh = mesh_assets.add(
        Mesh::from(shape::Icosphere { radius: 0.2, subdivisions: 5 })
    );
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: player_mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..Default::default()
            }),
            transform: Transform::from_xyz(1.0, 0.2, 1.0),
            ..Default::default()
        })
        .insert(Player { x: 1.0, z: 1.0, direction: PlayerDirection::East });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 0.5, 1.0).looking_at(
            Vec3 { x: 2.0, y: 0.5, z: 1.0 },
            Vec3::Y
        ),
        ..Default::default()
    });

    let wall_mesh = mesh_assets.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));

    for (i, row) in game_state.map.mini_map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 1 {
                commands.spawn_bundle(PbrBundle {
                    mesh: wall_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::ANTIQUE_WHITE,
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(Vec3::new(j as f32, 0.5, i as f32)),
                    ..Default::default()
                });
            }
        }
    }

    let floor_size = game_state.map.mini_map.len().max(game_state.map.mini_map[0].len()) as f32;

    let floor_mesh = mesh_assets.add(
        Mesh::from(shape::Plane {
            size: floor_size + 1.0,
        })
    );

    commands.spawn_bundle(PbrBundle {
        mesh: floor_mesh,
        material: materials.add(StandardMaterial {
            base_color: Color::ALICE_BLUE,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(floor_size / 2.0, 0.0, floor_size / 2.0)),
        ..Default::default()
    });
}
