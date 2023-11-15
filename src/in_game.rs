use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{bundles::{self, PhysEntity}, components::{Momentum, Collider}, systems::physics::collisions};
use crate::GameState;
use crate::systems;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
pub struct InGame;

impl Plugin for InGame {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_avoidee,
                spawn_avoider,
            )
        )
        .add_systems(
            Update,
            (
                systems::physics::apply_momentum,
                systems::controls::player_controls,
                loop_space,
                collisions,
            ).run_if(in_state(GameState::InGame))
        );
    }
}


fn spawn_avoidee(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    info!("Spawn avoidee");
    commands.spawn(bundles::Avoidee{
        mesh: MaterialMesh2dBundle{
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..Default::default()
        },
        physics: PhysEntity{
            momentum: Momentum(Vec2 { x:10., y: 10. }),
            collider: Collider::Circle { radius: 10. },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_avoider(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    info!("Spawning avoider");
    commands.spawn(bundles::Avoider{
        mesh: MaterialMesh2dBundle{
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform{
                translation: Vec3 { x: 20., y: 20., z: 0. },
                ..Default::default()
            },
            ..Default::default()
        },
        physics: PhysEntity{
            momentum: Momentum(Vec2 { x:10., y: 10. }),
            collider: Collider::Circle { radius: 10. },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn loop_space(
    mut query: Query<&mut Transform, With<Momentum>>
){
    for mut t in query.iter_mut(){
        if t.translation.y < -SCREEN_HEIGHT/2. {t.translation.y = SCREEN_HEIGHT/2.;}
        if t.translation.y > SCREEN_HEIGHT/2. {t.translation.y = -SCREEN_HEIGHT/2.;}
        if t.translation.x < -SCREEN_WIDTH/2. {t.translation.x = SCREEN_WIDTH/2.;}
        if t.translation.x > SCREEN_WIDTH/2. {t.translation.x = -SCREEN_WIDTH/2.;}
    }
}