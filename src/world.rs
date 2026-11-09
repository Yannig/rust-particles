use crate::constraint::Constraint;
use crate::particle::Particle;
use crate::point2d::Point2d;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub const TIME_STEP: f64 = 1.0 / 60.0;
const SOLVER_ITERATIONS: usize = 80;
// Displacement due to gravity per fixed step (acceleration * TIME_STEP^2).
const GRAVITY: Point2d = Point2d::new(0.0, 0.1);

pub struct World {
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub min: Point2d,
    pub max: Point2d,
    pub gravity: Point2d,
}

impl World {
    pub fn new(width: f64, height: f64) -> World {
        assert!(width.is_finite() && width > 0.0);
        assert!(height.is_finite() && height > 0.0);
        World {
            particles: Vec::new(),
            constraints: Vec::new(),
            min: Point2d::new(0.0, 0.0),
            max: Point2d::new(width, height),
            gravity: GRAVITY,
        }
    }

    pub fn add_particle(&mut self, p: Particle) -> usize {
        let index = self.particles.len();
        self.particles.push(p);
        index
    }

    pub fn add_constraint(&mut self, p1: usize, p2: usize, min_distance: f64, max_distance: f64) {
        assert!(p1 < self.particles.len() && p2 < self.particles.len() && p1 != p2);
        assert!(min_distance.is_finite() && max_distance.is_finite());
        assert!(min_distance >= 0.0 && max_distance >= min_distance);
        self.constraints.push(Constraint {
            p1,
            p2,
            min_distance,
            max_distance,
        });
    }

    pub fn create_rope(&mut self, start: Point2d, end: Point2d, node_count: usize) {
        assert!(node_count >= 2, "a rope needs at least two nodes");
        assert!([start.x, start.y, end.x, end.y]
            .iter()
            .all(|v| v.is_finite()));
        let step = (end - start) / (node_count - 1) as f64;
        let length = step.x.hypot(step.y);
        assert!(
            length > 0.0 && length.is_finite(),
            "a rope needs a nonzero length"
        );
        let first = self.particles.len();
        for i in 0..node_count {
            let fraction = i as f64 / (node_count - 1) as f64;
            let position = Point2d::new(
                start.x + (end.x - start.x) * fraction,
                start.y + (end.y - start.y) * fraction,
            );
            self.add_particle(Particle::new(
                position.x,
                position.y,
                0.0,
                0.0,
                if i == 0 { 0.0 } else { 1.0 },
                0.0,
                0.2,
            ));
            if i > 0 {
                self.add_constraint(first + i - 1, first + i, length, length);
            }
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(60, 100, 180));
        for c in &self.constraints {
            let p1 = self.particles[c.p1].current;
            let p2 = self.particles[c.p2].current;
            canvas.draw_line(
                Point::new(p1.x as i32, p1.y as i32),
                Point::new(p2.x as i32, p2.y as i32),
            )?;
        }
        for p in &self.particles {
            p.draw(canvas)?;
        }
        Ok(())
    }

    pub fn update(&mut self) {
        for p in &mut self.particles {
            p.update(&self.gravity);
        }
        for _ in 0..SOLVER_ITERATIONS {
            for c in &self.constraints {
                c.fix(&mut self.particles);
            }
            // Reapply boundaries because distance projection can push nodes outside.
            for p in &mut self.particles {
                p.check_rebound_x(self.min.x, self.max.x);
                p.check_rebound_y(self.min.y, self.max.y);
            }
        }
    }

    pub fn x(&self) -> u32 {
        self.max.x as u32
    }

    pub fn y(&self) -> u32 {
        self.max.y as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rope_connects_world_particles_and_reaches_endpoint() {
        let mut world = World::new(800.0, 600.0);
        world.add_particle(Particle::new(1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
        world.create_rope(Point2d::new(100.0, 50.0), Point2d::new(400.0, 50.0), 31);
        assert_eq!(world.particles.len(), 32);
        assert_eq!(world.constraints.len(), 30);
        assert_eq!(world.particles[31].current, Point2d::new(400.0, 50.0));
        assert_eq!(world.constraints[0].p1, 1);
        assert_eq!(world.constraints[29].p2, 31);
        world.update();
        assert_eq!(world.particles[1].current, Point2d::new(100.0, 50.0));
        assert!(world.particles[31].current.y > 50.0);
    }

    #[test]
    fn rope_stays_finite_anchored_and_nearly_inextensible() {
        let mut world = World::new(800.0, 600.0);
        let anchor = Point2d::new(300.0, 50.0);
        world.create_rope(anchor, Point2d::new(600.0, 150.0), 31);
        for _ in 0..3600 {
            world.update();
            assert_eq!(world.particles[0].current, anchor);
            for p in &world.particles {
                assert!(p.current.x.is_finite() && p.current.y.is_finite());
                assert!((0.0..=800.0).contains(&p.current.x));
                assert!((0.0..=600.0).contains(&p.current.y));
            }
            for c in &world.constraints {
                let delta = world.particles[c.p2].current - world.particles[c.p1].current;
                let relative_error = (delta.x.hypot(delta.y) / c.max_distance - 1.0).abs();
                assert!(relative_error < 0.05, "segment error: {relative_error}");
            }
        }
    }

    #[test]
    fn rope_collides_with_floor() {
        let mut world = World::new(200.0, 100.0);
        world.create_rope(Point2d::new(50.0, 20.0), Point2d::new(150.0, 20.0), 11);
        let mut touched_floor = false;
        for _ in 0..1200 {
            world.update();
            for p in &world.particles {
                assert!((0.0..=200.0).contains(&p.current.x));
                assert!((0.0..=100.0).contains(&p.current.y));
                touched_floor |= p.current.y == 100.0;
            }
        }
        assert!(touched_floor);
    }

    #[test]
    #[should_panic(expected = "at least two nodes")]
    fn rejects_rope_without_segments() {
        World::new(100.0, 100.0).create_rope(Point2d::new(10.0, 10.0), Point2d::new(20.0, 20.0), 1);
    }
}
