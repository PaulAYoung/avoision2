use bevy::ecs::system::Combine;
use bevy::prelude::*;

use crate::GameState;
use crate::components::ResetMarker;
use crate::in_game::Score;

pub struct GameOver;

impl Plugin for GameOver{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver),
            game_over_text,
        )
        .add_systems(
            Update,
            restart_game_on_space.run_if(in_state(GameState::GameOver))
        )
        ;
    }
}

fn game_over_text(mut commands: Commands, score: Res<Score>,
){
    commands.spawn((
        ResetMarker,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::new(
                "Game Over\n",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::new(
                format!("Score: {}", score.0.elapsed().as_secs()),
                TextStyle {
                    font_size: 40.0,
                    ..default()
                },
            ),
            TextSection::new(
                "\n\n\nSpace, click, or tap to try again",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
        ])
    ));
}

fn restart_game_on_space(
    mut commands: Commands,
    query: Query<Entity, With<ResetMarker>>,
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

        for e in query.iter(){
            commands.entity(e).despawn();
        }
    }
}