use std::ops::Range;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, collide_aabb::Collision}, time::Stopwatch, reflect::Array};

use rand::Rng;
use rand::seq::SliceRandom;

use crate::{bundles::{self, PhysEntity}, components::{Momentum, Collider, Avoidee, ResetMarker, Player}, systems::physics::{collisions, CollisionEvent}};
use crate::GameState;
use crate::systems;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

const AVOIDEE_POS_RANGE_X: Range<f32> = (SCREEN_WIDTH/4.)..(SCREEN_WIDTH/2.);
const AVOIDEE_POS_RANGE_Y: Range<f32> = (SCREEN_HEIGHT/4.)..(SCREEN_HEIGHT/2.);
const AVOIDEE_MOMENTUM_RANGE: Range<f32> = (-100.)..(100.);
const ONE_NEG_ONE: [f32; 2] = [1., -1.];
pub struct InGame;


#[derive(Resource)]
pub struct Score(pub Stopwatch);

#[derive(Resource)]
struct AvoiderSpawnTimer(Timer);

impl Default for AvoiderSpawnTimer{
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Repeating))
    }
}

impl Plugin for InGame {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Score(Stopwatch::new()))
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_avoider,
                spawn_avoidee,
                spawn_score_text,
                reset_score,
            )
        )
        .add_systems(
            Update,
            (
                systems::physics::apply_momentum,
                systems::controls::player_controls_keyboard,
                systems::controls::player_controls_mouse,
                loop_space,
                collisions,
                update_score,
                game_over_check,
                spawn_avoidee_timer,
            ).run_if(in_state(GameState::InGame))
        );
    }
}


fn spawn_avoidee(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Player>>,
){
    info!("Spawn avoidee");
    let mut rng = rand::thread_rng();
    let t_mod = Vec3 {
        x: rng.gen_range(AVOIDEE_POS_RANGE_X) * ONE_NEG_ONE.choose(&mut rng).unwrap(),
        y: rng.gen_range(AVOIDEE_POS_RANGE_Y) * ONE_NEG_ONE.choose(&mut rng).unwrap(),
        z: 0.
    };
    let avoidee_translatioin: Vec3;
    let avoidee_translation = match query.get_single(){
        Ok(player) => player.translation + t_mod,
        _ => t_mod,
    };

    commands.spawn((
        ResetMarker,
        bundles::Avoidee{
        mesh: MaterialMesh2dBundle{
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform{
                translation: avoidee_translation,
                ..Default::default()
            },
            ..Default::default()
        },
        physics: PhysEntity{
            momentum: Momentum(Vec2 { x: rng.gen_range(AVOIDEE_MOMENTUM_RANGE), y: rng.gen_range(AVOIDEE_MOMENTUM_RANGE)}),
            collider: Collider::Circle { radius: 10. },
            ..Default::default()
        },
        ..Default::default()
    }));
}

fn spawn_avoidee_timer(
    time: Res<Time>,
    mut spawn_timer: Local<AvoiderSpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Player>>,
){
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished(){
        spawn_avoidee(commands, meshes, materials, query);
    }
}

fn spawn_avoider(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    info!("Spawning avoider");
    commands.spawn((
        ResetMarker,
        Player,
        bundles::Avoider{
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
    }));
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

fn reset_score(mut score: ResMut<Score>){
    score.0.reset();
    score.0.unpause();
}

#[derive(Component)]
pub struct ScoreText;

fn spawn_score_text(mut commands: Commands){
    commands.spawn((
        ResetMarker,
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

fn game_over_check(
    query: Query<Entity, With<Player>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>
){
    let player_entity = query.single();

    for event in collision_events.read() {
        if (event.entity1 == player_entity || event.entity2 == player_entity){
            next_state.set(GameState::GameOver);
        }
    }
}