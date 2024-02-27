use bevy::prelude::*;
use bevy_renet::{ run_if_client_connected, RenetClientPlugin };
use network::*;
use player::*;
use store::game::*;
use setup::*;

mod network;
mod setup;
mod player;

fn main() {
    let username = String::from("Harry");

    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("TicTacTussle <{}>", username),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
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
        //setup
        .add_startup_system(setup)
        //add systems
        .add_system(player_movement)
        .add_system(player_rotation)
        // Finally we run the thing!
        .run();
}
