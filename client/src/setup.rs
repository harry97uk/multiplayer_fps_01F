use bevy::prelude::*;
use store::game::GameState;

pub fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_state: Res<GameState>
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 0.0, 1.0).looking_at(
            Vec3 { x: 2.0, y: 0.0, z: 1.0 },
            Vec3::Y
        ),
        ..Default::default()
    });

    let mesh = mesh_assets.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));

    for (i, row) in game_state.map.mini_map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 1 {
                commands.spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(StandardMaterial::from(Color::ANTIQUE_WHITE)),
                    transform: Transform::from_translation(Vec3::new(j as f32, 0.0, i as f32)),
                    ..Default::default()
                });
            }
        }
    }
}
