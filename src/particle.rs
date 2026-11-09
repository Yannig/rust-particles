use crate::point2d::Point2d;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[derive(Clone, Debug)]
pub struct Particle {
    pub current: Point2d,
    pub previous: Point2d,
    pub mass: f64,
    pub elasticity_factor: f64,
    pub friction_factor: f64,
}

impl Particle {
    pub fn new(
        x: f64,
        y: f64,
        speed_x: f64,
        speed_y: f64,
        mass: f64,
        elasticity_factor: f64,
        friction_factor: f64,
    ) -> Particle {
        assert!([x, y, speed_x, speed_y, mass].iter().all(|v| v.is_finite()));
        assert!((0.0..=1.0).contains(&elasticity_factor));
        assert!((0.0..=1.0).contains(&friction_factor));
        Particle {
            current: Point2d { x, y },
            previous: Point2d {
                x: x - speed_x,
                y: y - speed_y,
            },
            elasticity_factor,
            mass,
            friction_factor,
        }
    }
    pub fn inverse_mass(&self) -> f64 {
        if self.mass > 0.0 {
            1.0 / self.mass
        } else {
            0.0
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(if self.mass <= 0.0 {
            Color::RGB(200, 50, 50)
        } else {
            Color::RGB(0, 0, 0)
        });
        self.current.draw(canvas)
    }
    pub fn update(&mut self, gravity: &Point2d) {
        if self.mass <= 0.0 {
            return;
        }
        let _previous = self.current;
        self.current += &(self.current - self.previous + gravity);
        self.previous = _previous;
    }
    pub fn check_rebound_x(&mut self, min_x: f64, max_x: f64) {
        if self.mass <= 0.0 {
            return;
        }
        if self.current.x > max_x || self.current.x < min_x {
            let velocity = self.current - self.previous;
            let outward = (self.current.x > max_x && velocity.x > 0.0)
                || (self.current.x < min_x && velocity.x < 0.0);
            self.current.x = self.current.x.clamp(min_x, max_x);
            if outward {
                self.previous.x = self.current.x + self.elasticity_factor * velocity.x;
                self.previous.y = self.current.y - (1.0 - self.friction_factor) * velocity.y;
            }
        }
    }
    pub fn check_rebound_y(&mut self, min_y: f64, max_y: f64) {
        if self.mass <= 0.0 {
            return;
        }
        if self.current.y > max_y || self.current.y < min_y {
            let velocity = self.current - self.previous;
            let outward = (self.current.y > max_y && velocity.y > 0.0)
                || (self.current.y < min_y && velocity.y < 0.0);
            self.current.y = self.current.y.clamp(min_y, max_y);
            if outward {
                self.previous.x = self.current.x - (1.0 - self.friction_factor) * velocity.x;
                self.previous.y = self.current.y + self.elasticity_factor * velocity.y;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verlet_preserves_inertia_and_adds_gravity() {
        let mut p = Particle::new(1.0, 10.0, 2.0, 3.0, 1.0, 0.0, 0.0);
        p.update(&Point2d::new(0.0, 0.5));
        assert_eq!(p.previous, Point2d::new(1.0, 10.0));
        assert_eq!(p.current, Point2d::new(3.0, 13.5));
        p.update(&Point2d::new(0.0, 0.5));
        assert_eq!(p.current, Point2d::new(5.0, 17.5));
    }

    #[test]
    fn anchors_ignore_gravity_and_collisions() {
        for mass in [-1.0, 0.0] {
            let mut p = Particle::new(10.0, 10.0, 0.0, 0.0, mass, 1.0, 0.0);
            p.update(&Point2d::new(0.0, 1.0));
            p.check_rebound_x(0.0, 5.0);
            p.check_rebound_y(0.0, 5.0);
            assert_eq!(p.current, Point2d::new(10.0, 10.0));
        }
    }

    #[test]
    fn walls_project_and_reflect_normal_velocity() {
        for (start, velocity, boundary) in [(9.0, 4.0, 10.0), (1.0, -4.0, 0.0)] {
            let mut p = Particle::new(start, 5.0, velocity, 2.0, 1.0, 0.5, 0.25);
            p.update(&Point2d::new(0.0, 0.0));
            p.check_rebound_x(0.0, 10.0);
            assert_eq!(p.current.x, boundary);
            assert_eq!(p.current.x - p.previous.x, -velocity * 0.5);
            assert_eq!(p.current.y - p.previous.y, 1.5);
        }
    }

    #[test]
    fn floor_contact_can_stop_without_bouncing() {
        let mut p = Particle::new(5.0, 9.0, 2.0, 4.0, 1.0, 0.0, 1.0);
        p.update(&Point2d::new(0.0, 0.0));
        p.check_rebound_y(0.0, 10.0);
        assert_eq!(p.current, Point2d::new(7.0, 10.0));
        assert_eq!(p.previous, p.current);
    }
}
