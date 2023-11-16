use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::Stopwatch};

use crate::{bundles::{self, PhysEntity}, components::{Momentum, Collider}, systems::physics::collisions};
use crate::GameState;
use crate::systems;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct InGame;

#[derive(Resource)]
pub struct Score(Stopwatch);

impl Plugin for InGame {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Score(Stopwatch::new()))
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_avoidee,
                spawn_avoider,
                spawn_score_text,
                rest_score,
            )
        )
        .add_systems(
            Update,
            (
                systems::physics::apply_momentum,
                systems::controls::player_controls,
                loop_space,
                collisions,
                update_score,
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

fn rest_score(mut score: ResMut<Score>){
    score.0.reset();
    score.0.unpause();
}

#[derive(Component)]
pub struct ScoreText;

fn spawn_score_text(mut commands: Commands){
    commands.spawn((
        ScoreText,
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Start: 0",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 12.0,
                ..default()
            },
        )
    ));
}

fn update_score(
    mut score: ResMut<Score>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<ScoreText>>
){
    score.0.tick(time.delta());
    let mut score_text = query.get_single_mut().expect("Did not find score text.");
    score_text.sections[0].value = format!("Score: {}", score.0.elapsed().as_secs().to_string());
}