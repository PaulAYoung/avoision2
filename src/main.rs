use bevy::prelude::*;

mod bundles;
mod components;
mod in_game;
mod systems;

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
enum GameState{
    #[default]
    InGame,
    Menu,
    Pause,
}

fn setup(mut commands: Commands){
    info!("In setup");
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_plugin(in_game::InGame)
        .run();
}
