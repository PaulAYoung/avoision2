use bevy::prelude::*;

use crate::components;

const THRUST:f32 = 10.;

pub fn player_controls(
    mut query: Query<&mut components::Momentum, With<components::Avoider>>,
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