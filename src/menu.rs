use bevy::{prelude::*, transform::commands};
use crate::GameState;

pub struct Menu;

#[derive(Component)]
pub struct MenuEntity;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (
                spawn_menu_text,
            )
        )
        .add_systems(Update, (
            start_game_on_space
        ).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu),
            exit_menu
        )
        ;
    }
}

fn spawn_menu_text(mut commands: Commands){
    commands.spawn((
        MenuEntity,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Avoision 2",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
                ..default()
            },
        )
    ));
}

fn start_game_on_space(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>
){
    if keys.pressed(KeyCode::Space){
        next_state.set(GameState::InGame);
    }
}

fn exit_menu(
    mut commands: Commands,
    query: Query<Entity, With<MenuEntity>>
){
    for e in query.iter(){
        commands.entity(e).despawn_recursive();
    }
}