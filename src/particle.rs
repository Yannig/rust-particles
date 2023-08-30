use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

#[derive(Default)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub p_x: f64,
    pub p_y: f64,
    pub mass: f64,
    pub elasticity_factor: f64,
    pub friction_factor: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64, speed_x: f64, speed_y: f64, mass: f64, elasticity_factor: f64, friction_factor: f64) -> Particle {
        Particle {
            x,
            y,
            p_x: x - speed_x,
            p_y: y - speed_y,
            elasticity_factor,
            mass,
            friction_factor
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _x = self.x as i32;
        let _y = self.y as i32;
        let _ = canvas.draw_point(Point::new(_x, _y));
        let _ = canvas.draw_line(Point::new(_x - 2, _y - 2), Point::new(_x + 2, _y + 2));
        let _ = canvas.draw_line(Point::new(_x + 2,_y - 2), Point::new(_x - 2,_y + 2));
        println!("{}: {}", _x, _y)
    }
    pub fn update(&mut self, gravity: f64) {
        // Object with no mass are non movable objects
        if self.mass < 0.0 {
            return;
        }
        // Compute new position for current particle using previous position and gravity
        let p_x = self.x;
        let p_y = self.y;
        self.x = self.x + (self.x - self.p_x);
        self.y = self.y + (self.y - self.p_y) + gravity;
        self.p_x = p_x;
        self.p_y = p_y;
    }
    pub fn check_rebound_x(&mut self, min_x: f64, max_x: f64) {
        if self.x > max_x || self.x < min_x {
            let _x = self.x;
            self.x = self.p_x;
            self.p_x = _x - self.elasticity_factor * (_x - self.p_x);
            self.p_y = self.y - (1.0 - self.friction_factor) * (self.y - self.p_y);
        }
    }
    pub fn check_rebound_y(&mut self, min_y: f64, max_y: f64) {
        if self.y > max_y || self.y < min_y {
            let _y = self.y;
            self.y = self.p_y;
            self.p_x = self.x - (1.0 - self.friction_factor) * (self.x - self.p_x);
            self.p_y = _y - self.elasticity_factor * (_y - self.p_y);
        }
    }
}
