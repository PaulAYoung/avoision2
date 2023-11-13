use bevy::{prelude::*, transform::commands};
use crate::GameState;

pub struct Menu;

pub struct Menu_Text;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (
                spawn_menu_text,   
            )
        );
    }
}

pub fn spawn_menu_text(mut commands: Commands){
    commands.spawn((
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
