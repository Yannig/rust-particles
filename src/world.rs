use sdl2::render::WindowCanvas;
use crate::particle::Particle;


const GRAVITY: f64 = 0.1;

pub struct World {
    particles: Vec<Particle>,
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
            min_x: 0.0,
            min_y: 0.0,
            max_x: width,
            max_y: height,
            gravity: GRAVITY
        }
    }
    pub fn create_particle<'a>(&mut self, x: f64, y: f64, speed_x: f64, speed_y: f64, mass: f64, elasticity_factor: f64, friction_factor: f64) {
        self.particles.push(Particle::new(x, y, speed_x, speed_y, mass, elasticity_factor, friction_factor));
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for p in &self.particles {
            p.draw(canvas);
        }
    }
    pub fn update(&mut self) {
        for p in &mut self.particles {
            p.update(self.gravity);
            p.check_rebound_x(self.min_x, self.max_x);
            p.check_rebound_y(self.min_y, self.max_y);
        }
    }
    pub fn x(&self) -> u32 {
        return self.max_x as u32;
    }
    pub fn y(&self) -> u32 {
        return self.max_y as u32;
    }
}
