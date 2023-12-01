use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::{self, Momentum}, constants::{SCREEN_WIDTH, SCREEN_HEIGHT}};

const THRUST:f32 = 40.;

pub fn player_controls_touch(
    mut player: Query<(&mut components::Momentum, &Transform), With<components::Player>>,
    time: Res<Time>,
    touches: Res<Touches>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
){
    if let Some(touch_pos) = touches.first_pressed_position(){
        let window = q_windows.single();
        let cam_transform = q_camera.single();
        let (mut momentum, pos) = player.single_mut();
        let click_pos = window_cords_to_game_cords(
            window,
            &cam_transform.translation().xy(),
            &touch_pos);
        let thrust = (pos.translation.xy()-click_pos)
        .normalize_or_zero()*THRUST*time.delta_seconds();

        momentum.x -= thrust.x;
        momentum.y -= thrust.y;
        //info!( mouse_x=mouse_pos.x, mouse_y=mouse_pos.y);
        //player_x=pos.translation.x, player_y=pos.translation.y,
        //info!(thrust_x=thrust.x, thrust_y=thrust.y);
        //info!(cam_x=cam_transform.translation().x, cam_y=cam_transform.translation().y);
        //info!(click_x=click_pos.x, click_y=click_pos.y);
        //momentum_x=momentum.x, momentum_y=momentum.y
    }
}


pub fn player_controls_mouse(
    mut player: Query<(&mut components::Momentum, &Transform), With<components::Player>>,
    time: Res<Time>,
    mouse_button: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
){
    if mouse_button.pressed(MouseButton::Left){
        let window = q_windows.single();
        if let Some(mouse_pos) = window.cursor_position(){
            let cam_transform = q_camera.single();
            let (mut momentum, pos) = player.single_mut();
            let click_pos = window_cords_to_game_cords(
                window,
                &cam_transform.translation().xy(),
                &mouse_pos);
            let thrust = (pos.translation.xy()-click_pos)
            .normalize_or_zero()*THRUST*time.delta_seconds();

            momentum.x -= thrust.x;
            momentum.y -= thrust.y;
            //info!( mouse_x=mouse_pos.x, mouse_y=mouse_pos.y);
            //player_x=pos.translation.x, player_y=pos.translation.y,
            //info!(thrust_x=thrust.x, thrust_y=thrust.y);
            //info!(cam_x=cam_transform.translation().x, cam_y=cam_transform.translation().y);
            //info!(click_x=click_pos.x, click_y=click_pos.y);
            //momentum_x=momentum.x, momentum_y=momentum.y
        }
    }
}

fn window_cords_to_game_cords(window: &Window, camera_pos: &Vec2, window_pos: &Vec2)->Vec2{
    Vec2::new(
        window_pos.x-(window.resolution.width()/2.),
        -window_pos.y+(window.resolution.height()/2.)
    )+*camera_pos
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