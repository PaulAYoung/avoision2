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
            start_game
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
        TextBundle::from_sections([
            TextSection::new(
                "Avoision 2",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                }
            ),
            TextSection::new(
                "\n\n\nAvoid the red balls! How long can you survive?",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                }
            ),
            TextSection::new(
                "\n\n\nUse arrows keys, mouse, or touch screen to control.",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                }
            ),
            TextSection::new(
                "\n\n\nSpace, click, or tap to begin!",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 20.0,
                    ..default()
                }
            ),
        ])
    ));
}

fn start_game(
    keys: Res<Input<KeyCode>>,
    click: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>
){
    if keys.pressed(KeyCode::Space)
        || click.just_released(MouseButton::Left)
        || touches.any_just_pressed()
    {
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