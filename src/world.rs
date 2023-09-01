use std::rc::Rc;
use sdl2::render::WindowCanvas;
use crate::particle::Particle;
use crate::constraint::Constraint;

const GRAVITY: f64 = 0.1;

pub struct World {
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub gravity: f64
}

impl World {
    pub fn new(width: f64, height: f64) -> World {
        World {
            particles: Vec::new(),
            constraints: vec![],
            min_x: 0.0,
            min_y: 0.0,
            max_x: width,
            max_y: height,
            gravity: GRAVITY
        }
    }
    pub fn add_particle(&mut self, p: Particle) -> &mut Particle {
        self.particles.push(p);
        // Send back a reference over last pushed particle
        let i = self.particles.len() - 1;
        return &mut self.particles[i];
    }
    pub fn add_constraint(&'static mut self, p1: Rc<Particle>, p2: Rc<Particle>, min_distance: f64, max_distance: f64) {
        self.constraints.push(Constraint { p1, p2, min_distance, max_distance });
    }
    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        for p in &mut self.particles {
            p.draw(canvas);
        }
    }
    pub fn update(&mut self) {
        for p in &mut self.particles {
            p.update(self.gravity);
            p.check_rebound_x(self.min_x, self.max_x);
            p.check_rebound_y(self.min_y, self.max_y);
        }
        // Resolve constraints by looping
        for _i in 0..10 {
            for c in &mut self.constraints {
                if ! c.is_ok() {
                    c.fix();
                }
            }
        }
    }
    pub fn x(&self) -> u32 {
        return self.max_x as u32;
    }
    pub fn y(&self) -> u32 {
        return self.max_y as u32;
    }
}
