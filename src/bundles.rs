use bevy::{prelude::*, ecs::bundle};

use crate::components;

#[derive(Bundle, Default)]
pub struct PhysEntity{
    momentum: components::Momentum,
    collider: components::Collider,
}

#[derive(Bundle, Default)]
pub struct Avoidee{
    avoidee: components::Avoidee,
    #[bundle]
    physics:PhysEntity,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct Avoider{
    avoidee: components::Avoider,
    #[bundle]
    physics:PhysEntity,
    #[bundle]
    sprite: SpriteBundle,
}