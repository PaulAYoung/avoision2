use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{bundles::{self, PhysEntity}, components::{Momentum, Collider}, systems::physics::collisions};
use crate::GameState;
use crate::systems;
pub struct InGame;

impl Plugin for InGame {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                spawn_avoidee,
                spawn_avoider,
            ).in_schedule(OnEnter(GameState::InGame))
        )
        .add_systems(
            (
                apply_momentum,
                systems::controls::player_controls,
                collisions,
            ).in_set(OnUpdate(GameState::InGame))
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

fn apply_momentum(
    mut query: Query<(&mut Transform, &Momentum)>,
    time: Res<Time>
){
    let time_mod = time.delta_seconds();
    for (mut transform, momentum) in query.iter_mut(){
        transform.translation.x += (momentum.x*time_mod);
        transform.translation.y += (momentum.y*time_mod);
    }
}