use bevy::{ prelude::* };
use store::game::*;

use super::components::Materials;

pub fn cleanup_map(mut commands: Commands) {
    println!("cleaning map up");
    commands.remove_resource::<PbrBundle>();
}

pub fn spawn_map(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: Res<Materials>,
    game_state: Res<GameState>
) {
    println!("spawning map");
    spawn_floor(&mut commands, &mut mesh_assets, &mut materials, &game_state);
    spawn_walls(&mut commands, &mut mesh_assets, &mut materials, &game_state);
}

fn spawn_floor(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    materials: &Res<Materials>,
    game_state: &Res<GameState>
) {
    let floor_size = game_state.map.mini_map.len().max(game_state.map.mini_map[0].len()) as f32;

    let floor_mesh = mesh_assets.add(
        Mesh::from(shape::Plane {
            size: floor_size + 1.0,
        })
    );

    commands.spawn_bundle(PbrBundle {
        mesh: floor_mesh,
        material: materials.floor_material.clone(),
        transform: Transform::from_translation(Vec3::new(floor_size / 2.0, 0.0, floor_size / 2.0)),
        ..Default::default()
    });
}

fn spawn_walls(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    materials: &Res<Materials>,
    game_state: &Res<GameState>
) {
    let wall_mesh = mesh_assets.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));

    for (i, row) in game_state.map.mini_map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 1 {
                commands.spawn_bundle(PbrBundle {
                    mesh: wall_mesh.clone(),
                    material: materials.wall_material.clone(),
                    transform: Transform::from_translation(Vec3::new(j as f32, 0.5, i as f32)),
                    ..Default::default()
                });
            }
        }
    }
}
