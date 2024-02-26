use bevy::prelude::*;
use bevy_renet::{ run_if_client_connected, RenetClientPlugin };
use network::*;
use store::game::*;

mod network;

fn main() {
    let username = String::from("Harry");

    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("TicTacTussle <{}>", username),
            width: 480.0,
            height: 540.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
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
        // Finally we run the thing!
        .run();
}
