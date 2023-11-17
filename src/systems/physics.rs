use bevy::prelude::*;

use crate::components::{Collider, Momentum};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin{
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
    }
}
#[derive(Event)]
pub struct CollisionEvent{
    pub entity1: Entity,
    pub entity2: Entity
}

pub fn collides(c1: &Collider, c2: &Collider, p1:&Vec2, p2: &Vec2)->bool{
    match *c1 {
        Collider::Circle{radius: r} => {
            match *c2{
                Collider::Circle{radius: r2} => p1.distance(*p2) <= r+r2,
                Collider::Line { p1, p2 } => {
                    true
                }
            }
        },
        _ => collides(c2, c1, p2, p1)
    }
}


pub fn apply_momentum(
    mut query: Query<(&mut Transform, &Momentum)>,
    time: Res<Time>
){
    let time_mod = time.delta_seconds();
    for (mut transform, momentum) in query.iter_mut(){
        transform.translation.x += (momentum.x*time_mod);
        transform.translation.y += (momentum.y*time_mod);
    }
}

pub fn collisions(
    mut query: Query<(Entity, &mut Momentum, &mut Transform, &Collider)>,
    time: Res<Time>,
    mut collision_writer: EventWriter<CollisionEvent>
){
    // for [(mut m1, mut t1, c1), (mut m2, mut t2, c2)] in query.iter_combinations_mut(){

    // }
    let mut iter = query.iter_combinations_mut();
    while let Some([
        (e1, mut m1, mut t1, c1),
        (e2, mut m2, mut t2, c2)]) = iter.fetch_next(){
        let (p1, p2) = (t1.translation.truncate(), t2.translation.truncate());
        if collides(&c1, &c2, &p1, &p2){
            let m_mod = (p1-p2).normalize()*10.;
            m1.0 = m1.0 + m_mod;
            m2.0 = m2.0 - m_mod;
            collision_writer.send(CollisionEvent { entity1: e1, entity2: e2 });
        }
    }
}