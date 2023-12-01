use bevy::{
    prelude::*,
};

// game state modules
mod in_game;
mod menu;
mod game_over;

mod bundles;
mod components;
mod systems;
mod physics;

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
enum GameState{
    InGame,
    #[default]
    Menu,
    Pause,
    GameOver
}

fn setup(mut commands: Commands){
   //info!("In setup");
    commands.spawn(Camera2dBundle::default());
}

mod constants {
    pub const SCREEN_HEIGHT:f32 = 600.;
    pub const SCREEN_WIDTH:f32 = 600.;
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Avoision 2".into(),
                        resolution: (constants::SCREEN_HEIGHT, constants::SCREEN_WIDTH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        ))
        .add_state::<GameState>()
        .add_systems(
            Startup,
            setup
        )
        .add_plugins((
            in_game::InGame,
            menu::Menu,
            systems::physics::PhysicsPlugin,
            game_over::GameOver,
        ))
        .run();
}
