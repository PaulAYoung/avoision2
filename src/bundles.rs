use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components;

#[derive(Bundle, Default)]
pub struct PhysEntity{
    pub momentum: components::Momentum,
    pub collider: components::Collider,
    pub mass: components::Mass,
}

#[derive(Bundle, Default)]
pub struct Avoidee{
    pub avoidee: components::Avoidee,
    #[bundle]
    pub physics:PhysEntity,
    #[bundle]
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle, Default)]
pub struct Avoider{
    pub avoidee: components::Avoider,
    #[bundle]
    pub physics:PhysEntity,
    #[bundle]
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}