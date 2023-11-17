use bevy::{
    prelude::*,
    diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}
};

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
    GameOver
}

fn setup(mut commands: Commands){
    info!("In setup");
    commands.spawn(Camera2dBundle::default());
}

mod constants {
    pub const SCREEN_HEIGHT:f32 = 300.;
    pub const SCREEN_WIDTH:f32 = 300.;
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
        ))
        .run();
}
