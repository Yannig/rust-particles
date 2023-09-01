use std::rc::Rc;
use crate::particle::Particle;

pub struct Constraint {
    pub p1: Rc<Particle>,
    pub p2: Rc<Particle>,
    pub min_distance: f64,
    pub max_distance: f64,
}

impl Constraint {
    pub fn square_distance(&self) -> f64 {
        return 0.0;
        /* let x = self.p1.x - self.p2.x;
        let y = self.p1.y - self.p2.y;
        return x * x + y * y;*/
    }
    /* pub fn distance(&self) -> f64 {
        return f64::sqrt(self.square_distance());
    }*/
    pub fn is_ok(&self) -> bool {
        let square_distance = self.square_distance();
        let square_min_distance = self.min_distance * self.min_distance;
        let square_max_distance = self.max_distance * self.max_distance;
        return square_distance > square_min_distance && square_distance < square_max_distance;
    }
    pub fn fix(&mut self) {
        /*let vector_x = (self.p2.x - self.p1.x) / 2.0;
        let vector_y = (self.p2.y - self.p1.y) / 2.0;
        self.p1.x += vector_x;
        self.p2.x -= vector_x;
        self.p1.y += vector_y;
        self.p2.y -= vector_y;*/
    }
}
