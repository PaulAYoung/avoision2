use bevy::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Circle(f32);

#[derive(Default)]
pub struct Line{p1: Vec2, p2: Vec2}

impl Circle{
    pub fn overlaps_circle(&self, c: Circle, p1: Vec2, p2: Vec2)->bool{
        (p1-p2).length()<=self.0+c.0
    }
}

impl Line {
    pub fn intersects_line(&self, l: Line, p1: Vec2, p2: Vec2)->bool{
        true
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn circles_collide(){
        let c1 = Circle(10.);
        let c2 = Circle(10.);

        assert!(c1.overlaps_circle(c2, 
            Vec2::new(0., 0.),
            Vec2::new(5., 5.),
        ))
    }

    #[test]
    fn circles_dont_collide(){
        let c1 = Circle(10.);
        let c2 = Circle(10.);

        assert!(!c1.overlaps_circle(c2, 
            Vec2::new(100., 0.),
            Vec2::new(5., 5.),
        ))
    }

    #[test]
    fn lines_intersect(){
        let l1 = Line{p1: Vec2::new(-10., 10.), p2: Vec2::new(0., 0.)};
        let l2 = Line{p1: Vec2::new(10., 0.), p2: Vec2::new(0., 0.)};

        assert!(
            l1.intersects_line(l2, Vec2::new(1., 0.), Vec2::new(-1., 0.))
        )
    }

    #[test]
    fn lines_dont_intersect(){
        let l1 = Line{p1: Vec2::new(-10., 10.), p2: Vec2::new(0., 0.)};
        let l2 = Line{p1: Vec2::new(10., 0.), p2: Vec2::new(0., 0.)};

        assert!(
            !l1.intersects_line(l2, Vec2::new(1., 0.), Vec2::new(-1., 0.))
        )
    }
}