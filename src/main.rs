use bevy::prelude::*;

mod bundles;
mod components;
mod in_game;
mod menu;
mod systems;
mod physics;

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
enum GameState{
    InGame,
    #[default]
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
        .add_systems(
            Startup,
            (
                setup
            )
        )
        .add_plugins((
            in_game::InGame,
            menu::Menu
        ))
        .run();
}
