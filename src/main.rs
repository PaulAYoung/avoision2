use bevy::prelude::*;

mod bundles;
mod components;
enum GameState{
    InGame,
    Menu,
    Pause
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
