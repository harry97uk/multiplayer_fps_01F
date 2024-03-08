mod player;
mod map;
mod components;
use std::io;

use player::*;
use map::*;
use self::{ components::Materials };

use super::AppState;
use bevy::prelude::*;
use bevy_renet::{ run_if_client_connected, RenetClientPlugin };
use super::network::*;
use store::game::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("Enter username: ");
        let mut username = String::new();
        match io::stdin().read_line(&mut username) {
            Ok(_) => {
                println!("username read success: {}", username);
            }
            Err(error) => println!("error: {error}"),
        }

        app
            // Renet setup
            .add_plugin(RenetClientPlugin)
            .insert_resource(new_renet_client(&username).unwrap())
            .add_system(handle_renet_error)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                receive_events_from_server.with_run_criteria(run_if_client_connected)
            )
            // Add our game state and register GameEvent as a bevy event
            .insert_resource(GameState::new())
            .add_event::<GameEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_map))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_map))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(back_to_main_menu_controls)
            )
            .add_plugin(PlayerPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(Materials {
        player_material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..Default::default()
        }),
        floor_material: materials.add(StandardMaterial {
            base_color: Color::ALICE_BLUE,
            ..Default::default()
        }),
        wall_material: materials.add(StandardMaterial {
            base_color: Color::ANTIQUE_WHITE,
            ..Default::default()
        }),
    });
}

fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    if *app_state.current() == AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}
