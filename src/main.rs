use bevy::prelude::*;

mod bundles;
mod components;
mod in_game;
mod systems;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState{
    InGame,
    Menu,
    Pause
}

fn setup(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::InGame)
        .add_startup_system(setup)
        .add_plugin(in_game::InGame)
        .run();
}
