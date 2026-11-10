use crate::constraint::Constraint;
use crate::particle::Particle;
use crate::point2d::Point2d;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub const TIME_STEP: f64 = 1.0 / 60.0;
const SOLVER_ITERATIONS: usize = 10;
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
        assert!(
            p.radius.is_finite() && p.radius >= 0.0,
            "invalid particle radius"
        );
        assert!(
            p.radius <= (self.max.x - self.min.x).min(self.max.y - self.min.y) / 2.0,
            "particle does not fit in world"
        );
        let index = self.particles.len();
        self.particles.push(p);
        index
    }

    pub fn explode(&mut self, center: Point2d, radius: f64, impulse: f64) {
        assert!(center.x.is_finite() && center.y.is_finite());
        assert!(
            radius.is_finite() && radius > 0.0,
            "invalid explosion radius"
        );
        assert!(
            impulse.is_finite() && impulse >= 0.0,
            "invalid explosion impulse"
        );
        for p in &mut self.particles {
            if p.mass <= 0.0 {
                continue;
            }
            let delta = p.current - center;
            if delta.x.abs() >= radius || delta.y.abs() >= radius {
                continue;
            }
            let distance = delta.x.hypot(delta.y);
            if distance >= radius {
                continue;
            }
            let direction = if distance > 0.0 {
                delta / distance
            } else {
                Point2d::new(0.0, -1.0)
            };
            let kick = impulse * (1.0 - distance / radius) * p.inverse_mass();
            // Changing the previous position adds to Verlet's implicit velocity.
            p.previous.x -= direction.x * kick;
            p.previous.y -= direction.y * kick;
        }
    }

    pub fn add_constraint(&mut self, p1: usize, p2: usize, min_distance: f64, max_distance: f64) {
        assert!(p1 < self.particles.len() && p2 < self.particles.len() && p1 != p2);
        assert!(min_distance.is_finite() && max_distance.is_finite());
        assert!(min_distance >= 0.0 && max_distance >= min_distance);
        assert!(
            max_distance >= self.particles[p1].radius + self.particles[p2].radius,
            "distance constraint conflicts with particle radii"
        );
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
            self.add_particle(
                Particle::new(
                    position.x,
                    position.y,
                    0.0,
                    0.0,
                    if i == 0 { 0.0 } else { 1.0 },
                    0.0,
                    0.2,
                )
                .with_radius(3.0_f64.min(length / 2.0)),
            );
            if i > 0 {
                self.add_constraint(first + i - 1, first + i, length, length);
            }
        }
    }

    pub fn create_flag(
        &mut self,
        top_left: Point2d,
        bottom_right: Point2d,
        columns: usize,
        rows: usize,
    ) {
        assert!(
            columns >= 2 && rows >= 2,
            "a flag needs at least two rows and columns"
        );
        assert!([top_left.x, top_left.y, bottom_right.x, bottom_right.y]
            .iter()
            .all(|v| v.is_finite()));
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;
        assert!(width > 0.0 && height > 0.0, "invalid flag dimensions");
        let dx = width / (columns - 1) as f64;
        let dy = height / (rows - 1) as f64;
        let radius = 3.0_f64.min(dx / 2.0).min(dy / 2.0);
        assert!(
            top_left.x >= self.min.x + radius
                && top_left.y >= self.min.y + radius
                && bottom_right.x <= self.max.x - radius
                && bottom_right.y <= self.max.y - radius,
            "flag does not fit in world"
        );
        let first = self.particles.len();
        for row in 0..rows {
            for column in 0..columns {
                let anchored = row == rows - 1 && (column == 0 || column == columns - 1);
                self.add_particle(
                    Particle::new(
                        top_left.x + width * column as f64 / (columns - 1) as f64,
                        top_left.y + height * row as f64 / (rows - 1) as f64,
                        0.0,
                        0.0,
                        if anchored { 0.0 } else { 1.0 },
                        0.0,
                        0.2,
                    )
                    .with_radius(radius),
                );
                let index = first + row * columns + column;
                if column > 0 {
                    self.add_constraint(index - 1, index, 0.0, dx);
                }
                if row > 0 {
                    self.add_constraint(index - columns, index, 0.0, dy);
                }
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
            self.resolve_overlaps();
            // Reapply boundaries because distance projection can push nodes outside.
            for p in &mut self.particles {
                p.check_rebound_x(self.min.x, self.max.x);
                p.check_rebound_y(self.min.y, self.max.y);
            }
        }
    }

    fn resolve_overlaps(&mut self) {
        for p1 in 0..self.particles.len() {
            for p2 in p1 + 1..self.particles.len() {
                let min_distance = self.particles[p1].radius + self.particles[p2].radius;
                let delta = self.particles[p2].current - self.particles[p1].current;
                if delta.x.abs() >= min_distance || delta.y.abs() >= min_distance {
                    continue;
                }
                // A contact is a unilateral distance constraint, using the same mass weighting.
                Constraint {
                    p1,
                    p2,
                    min_distance,
                    max_distance: f64::MAX,
                }
                .fix(&mut self.particles);
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
    fn explosion_adds_radial_velocity_with_distance_and_mass_falloff() {
        let mut world = World::new(500.0, 500.0);
        for (x, y, mass) in [
            (150.0, 250.0, 1.0),
            (250.0, 150.0, 2.0),
            (275.0, 250.0, 1.0),
        ] {
            world.add_particle(disk(x, y, mass, 3.0));
        }
        world.explode(Point2d::new(250.0, 250.0), 200.0, 20.0);
        for (p, velocity) in world.particles.iter().zip([
            Point2d::new(-10.0, 0.0),
            Point2d::new(0.0, -5.0),
            Point2d::new(17.5, 0.0),
        ]) {
            assert_eq!(p.current - p.previous, velocity);
        }
    }

    #[test]
    fn explosion_preserves_anchors_and_particles_outside_radius() {
        let mut world = World::new(500.0, 500.0);
        for (x, y, mass) in [
            (250.0, 250.0, 0.0),
            (260.0, 250.0, -1.0),
            (350.0, 250.0, 1.0),
            (340.0, 340.0, 1.0),
        ] {
            world.add_particle(disk(x, y, mass, 3.0));
        }
        let before = world.particles.clone();
        world.explode(Point2d::new(250.0, 250.0), 100.0, 20.0);
        for (p, old) in world.particles.iter().zip(before) {
            assert_eq!(p.current, old.current);
            assert_eq!(p.previous, old.previous);
        }
    }

    #[test]
    fn explosion_at_center_kicks_upward_and_preserves_existing_velocity() {
        let mut world = World::new(500.0, 500.0);
        world.add_particle(Particle::new(250.0, 250.0, 3.0, 2.0, 1.0, 0.0, 0.0));
        world.explode(Point2d::new(250.0, 250.0), 100.0, 20.0);
        let p = &world.particles[0];
        assert_eq!(p.current, Point2d::new(250.0, 250.0));
        assert_eq!(p.current - p.previous, Point2d::new(3.0, -18.0));
    }

    #[test]
    fn flag_builds_flexible_grid_with_two_bottom_anchors() {
        let mut world = World::new(800.0, 600.0);
        world.add_particle(disk(10.0, 10.0, 1.0, 3.0));
        world.create_flag(Point2d::new(100.0, 100.0), Point2d::new(220.0, 180.0), 7, 5);
        assert_eq!(world.particles.len(), 36);
        assert_eq!(world.constraints.len(), 5 * 6 + 4 * 7);
        let anchors: Vec<_> = world
            .particles
            .iter()
            .enumerate()
            .filter(|(_, p)| p.mass == 0.0)
            .map(|(i, p)| (i, p.current))
            .collect();
        assert_eq!(
            anchors,
            vec![
                (29, Point2d::new(100.0, 180.0)),
                (35, Point2d::new(220.0, 180.0)),
            ]
        );
        assert_eq!(world.particles[1].current, Point2d::new(100.0, 100.0));
        assert!(world.constraints.iter().all(|c| c.p1 >= 1 && c.p2 >= 1));
        for c in &world.constraints {
            let first = c.p1 - 1;
            let second = c.p2 - 1;
            assert!((second == first + 1 && first / 7 == second / 7) || second == first + 7);
            assert_eq!(c.min_distance, 0.0);
            let delta = world.particles[c.p2].current - world.particles[c.p1].current;
            assert!((delta.x.hypot(delta.y) - c.max_distance).abs() < 1e-10);
        }
    }

    #[test]
    fn flag_falls_while_bottom_corners_remain_fixed() {
        let mut world = World::new(500.0, 500.0);
        world.create_flag(Point2d::new(100.0, 100.0), Point2d::new(220.0, 180.0), 7, 5);
        for _ in 0..600 {
            world.update();
            assert_eq!(world.particles[28].current, Point2d::new(100.0, 180.0));
            assert_eq!(world.particles[34].current, Point2d::new(220.0, 180.0));
            for p in &world.particles {
                assert!(p.current.x.is_finite() && p.current.y.is_finite());
                assert!((p.radius..=500.0 - p.radius).contains(&p.current.x));
                assert!((p.radius..=500.0 - p.radius).contains(&p.current.y));
            }
        }
        assert!(world.particles[3].current.y > 180.0);
    }

    #[test]
    fn flag_links_allow_compression_but_limit_stretching() {
        let mut world = World::new(500.0, 500.0);
        world.create_flag(Point2d::new(100.0, 100.0), Point2d::new(220.0, 180.0), 7, 5);
        let link = &world.constraints[0];
        let first = world.particles[link.p1].current;
        world.particles[link.p2].current = Point2d::new(first.x + 10.0, first.y);
        link.fix(&mut world.particles);
        assert_eq!(world.particles[link.p1].current, first);
        assert_eq!(
            world.particles[link.p2].current,
            Point2d::new(first.x + 10.0, first.y)
        );

        world.particles[link.p2].current.x = first.x + 40.0;
        link.fix(&mut world.particles);
        let delta = world.particles[link.p2].current - world.particles[link.p1].current;
        assert!((delta.x.hypot(delta.y) - link.max_distance).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "at least two rows and columns")]
    fn rejects_flag_without_cells() {
        World::new(500.0, 500.0).create_flag(
            Point2d::new(100.0, 100.0),
            Point2d::new(200.0, 200.0),
            1,
            5,
        );
    }

    #[test]
    #[should_panic(expected = "invalid flag dimensions")]
    fn rejects_inverted_flag() {
        World::new(500.0, 500.0).create_flag(
            Point2d::new(200.0, 200.0),
            Point2d::new(100.0, 100.0),
            5,
            5,
        );
    }

    #[test]
    #[should_panic(expected = "flag does not fit")]
    fn rejects_flag_outside_world() {
        World::new(100.0, 100.0).create_flag(
            Point2d::new(10.0, 10.0),
            Point2d::new(120.0, 80.0),
            5,
            5,
        );
    }

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
            assert_no_overlaps(&world, 0.02);
            for p in &world.particles {
                assert!(p.current.x.is_finite() && p.current.y.is_finite());
                assert!((p.radius..=800.0 - p.radius).contains(&p.current.x));
                assert!((p.radius..=600.0 - p.radius).contains(&p.current.y));
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
                assert!((p.radius..=200.0 - p.radius).contains(&p.current.x));
                assert!((p.radius..=100.0 - p.radius).contains(&p.current.y));
                touched_floor |= p.current.y + p.radius == 100.0;
            }
        }
        assert!(touched_floor);
    }

    #[test]
    #[should_panic(expected = "at least two nodes")]
    fn rejects_rope_without_segments() {
        World::new(100.0, 100.0).create_rope(Point2d::new(10.0, 10.0), Point2d::new(20.0, 20.0), 1);
    }

    fn disk(x: f64, y: f64, mass: f64, radius: f64) -> Particle {
        Particle::new(x, y, 0.0, 0.0, mass, 0.0, 0.0).with_radius(radius)
    }

    fn assert_no_overlaps(world: &World, tolerance: f64) {
        for (i, p1) in world.particles.iter().enumerate() {
            for p2 in &world.particles[i + 1..] {
                let delta = p2.current - p1.current;
                assert!(
                    delta.x.hypot(delta.y) + tolerance >= p1.radius + p2.radius,
                    "overlap between {:?} and {:?}",
                    p1.current,
                    p2.current
                );
            }
        }
    }

    #[test]
    fn overlaps_use_radii_and_inverse_masses() {
        let mut world = World::new(100.0, 100.0);
        world.gravity = Point2d::new(0.0, 0.0);
        world.add_particle(disk(40.0, 50.0, 1.0, 4.0));
        world.add_particle(disk(46.0, 50.0, 3.0, 6.0));
        world.update();
        assert_eq!(world.particles[0].current.x, 37.0);
        assert_eq!(world.particles[1].current.x, 47.0);
        assert_no_overlaps(&world, 1e-10);
    }

    #[test]
    fn coincident_disk_moves_away_from_anchor() {
        let mut world = World::new(100.0, 100.0);
        world.gravity = Point2d::new(0.0, 0.0);
        world.add_particle(disk(50.0, 50.0, 0.0, 8.0));
        world.add_particle(disk(50.0, 50.0, 1.0, 4.0));
        world.update();
        assert_eq!(world.particles[0].current, Point2d::new(50.0, 50.0));
        assert_no_overlaps(&world, 1e-10);
    }

    #[test]
    fn separated_and_touching_disks_are_unchanged() {
        let mut world = World::new(100.0, 100.0);
        world.gravity = Point2d::new(0.0, 0.0);
        for x in [20.0, 30.0, 50.0] {
            world.add_particle(disk(x, 50.0, 1.0, 5.0));
        }
        world.update();
        for (p, x) in world.particles.iter().zip([20.0, 30.0, 50.0]) {
            assert_eq!(p.current, Point2d::new(x, 50.0));
        }
    }

    #[test]
    fn approaching_disks_do_not_interpenetrate() {
        let mut world = World::new(100.0, 100.0);
        world.gravity = Point2d::new(0.0, 0.0);
        world.add_particle(Particle::new(35.0, 50.0, 2.0, 0.0, 1.0, 0.0, 0.0).with_radius(6.0));
        world.add_particle(Particle::new(65.0, 50.0, -2.0, 0.0, 1.0, 0.0, 0.0).with_radius(6.0));
        for _ in 0..120 {
            world.update();
            assert_no_overlaps(&world, 1e-10);
            assert!(world.particles[0].current.x < world.particles[1].current.x);
        }
    }

    #[test]
    fn short_rope_uses_compatible_radii() {
        let mut world = World::new(100.0, 100.0);
        world.create_rope(Point2d::new(20.0, 20.0), Point2d::new(24.0, 20.0), 5);
        for p in &world.particles {
            assert_eq!(p.radius, 0.5);
        }
        world.update();
        assert_no_overlaps(&world, 1e-6);
    }

    #[test]
    fn overlapping_anchors_remain_fixed() {
        let mut world = World::new(100.0, 100.0);
        world.add_particle(disk(50.0, 50.0, 0.0, 8.0));
        world.add_particle(disk(50.0, 50.0, -1.0, 4.0));
        world.update();
        for p in &world.particles {
            assert_eq!(p.current, Point2d::new(50.0, 50.0));
        }
    }

    #[test]
    fn stacked_disks_resolve_contacts_and_floor_together() {
        let mut world = World::new(100.0, 100.0);
        for y in [70.0, 78.0, 86.0] {
            world.add_particle(disk(50.0, y, 1.0, 6.0));
        }
        for _ in 0..600 {
            world.update();
            assert_no_overlaps(&world, 1e-6);
            for p in &world.particles {
                assert!(p.current.x >= p.radius && p.current.x <= 100.0 - p.radius);
                assert!(p.current.y >= p.radius && p.current.y <= 100.0 - p.radius);
            }
        }
    }

    #[test]
    fn rope_nodes_resolve_self_contacts() {
        let mut world = World::new(200.0, 100.0);
        world.create_rope(Point2d::new(50.0, 20.0), Point2d::new(150.0, 20.0), 11);
        world.add_particle(disk(90.0, 75.0, 0.0, 12.0));
        for _ in 0..1200 {
            world.update();
            assert_no_overlaps(&world, 0.02);
        }
    }

    #[test]
    #[should_panic(expected = "particle does not fit")]
    fn rejects_disk_larger_than_world() {
        World::new(10.0, 10.0).add_particle(disk(5.0, 5.0, 1.0, 6.0));
    }

    #[test]
    #[should_panic(expected = "conflicts with particle radii")]
    fn rejects_impossible_distance_constraint() {
        let mut world = World::new(100.0, 100.0);
        world.add_particle(disk(20.0, 20.0, 1.0, 6.0));
        world.add_particle(disk(30.0, 20.0, 1.0, 6.0));
        world.add_constraint(0, 1, 10.0, 10.0);
    }
}
