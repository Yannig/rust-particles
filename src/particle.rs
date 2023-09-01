use std::ops;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

#[derive(Clone, Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64
}

pub struct Particle {
    pub current: Point2d,
    pub previous: Point2d,
    pub mass: f64,
    pub elasticity_factor: f64,
    pub friction_factor: f64,
}

impl ops::Add<Point2d> for Point2d {
    type Output = Point2d;
    fn add(self, rhs: Point2d) -> Point2d {
        return Point2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::AddAssign<Point2d> for Point2d {
    fn add_assign(&mut self, rhs: Point2d) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl ops::Div<f64> for Point2d {
    type Output = Point2d;
    fn div(self, rhs: f64) -> Point2d {
        return Point2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Point2d {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let _x = self.x as i32;
        let _y = self.y as i32;
        let _ = canvas.draw_point(Point::new(_x, _y));
        let _ = canvas.draw_line(Point::new(_x - 2, _y - 2), Point::new(_x + 2, _y + 2));
        let _ = canvas.draw_line(Point::new(_x + 2,_y - 2), Point::new(_x - 2,_y + 2));
    }
}
impl Particle {
    pub fn new(x: f64, y: f64, speed_x: f64, speed_y: f64, mass: f64, elasticity_factor: f64, friction_factor: f64) -> Particle {
        Particle {
            current: Point2d {x, y},
            previous: Point2d {x: x - speed_x, y: y - speed_y},
            elasticity_factor,
            mass,
            friction_factor
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.current.draw(canvas);
    }
    pub fn update(&mut self, gravity: f64) {
        // Object with no mass are non movable objects
        if self.mass < 0.0 {
            return;
        }
        // Compute new position for current particle using previous position and gravity
        let _previous = self.current.clone();
        self.current += Point2d { x: self.current.x - self.previous.x, y: (self.current.y - self.previous.y) + gravity };
        self.previous = _previous;
    }
    pub fn check_rebound_x(&mut self, min_x: f64, max_x: f64) {
        if self.current.x > max_x || self.current.x < min_x {
            let _x = self.current.x;
            self.current.x = self.previous.x;
            self.previous.x = _x - self.elasticity_factor * (_x - self.previous.x);
            self.previous.y = self.current.y - (1.0 - self.friction_factor) * (self.current.y - self.previous.y);
        }
    }
    pub fn check_rebound_y(&mut self, min_y: f64, max_y: f64) {
        if self.current.y > max_y || self.current.y < min_y {
            let _y = self.current.y;
            self.current.y = self.previous.y;
            self.previous.x = self.current.x - (1.0 - self.friction_factor) * (self.current.x - self.previous.x);
            self.previous.y = _y - self.elasticity_factor * (_y - self.previous.y);
        }
    }
}
