use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::{self, Momentum}, constants::{SCREEN_WIDTH, SCREEN_HEIGHT}};

const THRUST:f32 = 40.;

pub fn player_controls_mouse(
    mut player: Query<(&mut components::Momentum, &Transform), With<components::Player>>,
    time: Res<Time>,
    mouse_button: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
){
    if mouse_button.pressed(MouseButton::Left){
        if let Some(mouse_pos) = q_windows.single().cursor_position(){
            let (mut momentum, pos) = player.single_mut();
            let thrust = Vec2::new(
                pos.translation.x - (mouse_pos.x-(SCREEN_WIDTH/2.)),
                pos.translation.y - (-mouse_pos.y+(SCREEN_HEIGHT/2.))
            ).normalize_or_zero()*THRUST*time.delta_seconds();
            momentum.x -= thrust.x;
            momentum.y -= thrust.y;
            //info!( mouse_x=mouse_pos.x, mouse_y=mouse_pos.y);
                //player_x=pos.translation.x, player_y=pos.translation.y,
            info!(thrust_x=thrust.x, thrust_y=thrust.y);
                //momentum_x=momentum.x, momentum_y=momentum.y
        }
    }
}
pub fn player_controls_keyboard(
    mut query: Query<&mut components::Momentum, With<components::Player>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>
){
    for mut m in query.iter_mut(){
        if keys.pressed(KeyCode::Up){
            m.y += THRUST * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Down){
            m.y -= THRUST * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Right){
            m.x += THRUST * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Left){
            m.x -= THRUST * time.delta_seconds();
        }
    }
}