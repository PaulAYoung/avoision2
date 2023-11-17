use bevy::prelude::{
    Component,
    Vec2, Deref, DerefMut
};

#[derive(Component)]
pub struct ResetMarker;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Avoidee;

#[derive(Component, Default)]
pub struct Avoider;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Momentum(pub Vec2);

#[derive(Component, PartialEq)]
pub struct Mass(pub f32);

impl Default for Mass{
    fn default() -> Self {
        Mass(1.)
    }
}

#[derive(Component)]
pub enum Collider{
    Circle{radius: f32},
    Line{p1: Vec2, p2: Vec2},
}

impl Default for Collider {
    fn default() -> Self {
        Collider::Circle { radius: (1.0) }
    }
}