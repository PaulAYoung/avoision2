use bevy::prelude::*;

use crate::components::{self, Collider, Momentum};

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
    mut query: Query<(&mut Momentum, &mut Transform, &Collider)>,
    time: Res<Time>
){
    // for [(mut m1, mut t1, c1), (mut m2, mut t2, c2)] in query.iter_combinations_mut(){

    // }
    let mut iter = query.iter_combinations_mut();
    while let Some([(mut m1, mut t1, c1), (mut m2, mut t2, c2)]) = iter.fetch_next(){
        let (p1, p2) = (t1.translation.truncate(), t2.translation.truncate());
        if collides(&c1, &c2, &p1, &p2){
            let m_mod = (p1-p2).normalize()*10.;
            m1.0 = m1.0 + m_mod;
            m2.0 = m2.0 - m_mod;
        }
    }
}