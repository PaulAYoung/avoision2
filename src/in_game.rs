use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{bundles::{self, PhysEntity}, components::{Momentum}};
use crate::GameState;
use crate::systems;
pub struct InGame;

impl Plugin for InGame {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(spawn_avoidee)
                .with_system(spawn_avoider)
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(apply_momentum)
                .with_system(systems::controls::player_controls)
        );
    }
}


fn spawn_avoidee(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(bundles::Avoidee{
        mesh: MaterialMesh2dBundle{
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..Default::default()
        },
        physics: PhysEntity{
            momentum: Momentum(Vec2 { x:10., y: 10. }),
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
    commands.spawn(bundles::Avoider{
        mesh: MaterialMesh2dBundle{
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..Default::default()
        },
        physics: PhysEntity{
            momentum: Momentum(Vec2 { x:10., y: 10. }),
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