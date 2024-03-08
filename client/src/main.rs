use bevy::prelude::*;
use main_menu::MainMenuPlugin;
use game::GamePlugin;

mod network;
mod helper;
mod game;
mod main_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("Maze Wars"),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Finally we run the thing!
        .run();
}
