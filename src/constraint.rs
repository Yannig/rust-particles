use crate::particle::Particle;
use crate::point2d::Point2d;

pub struct Constraint {
    pub p1: usize,
    pub p2: usize,
    pub min_distance: f64,
    pub max_distance: f64,
}

impl Constraint {
    pub fn fix(&self, particles: &mut [Particle]) {
        let delta = particles[self.p2].current - particles[self.p1].current;
        let distance_squared = delta.x * delta.x + delta.y * delta.y;
        let min_squared = self.min_distance * self.min_distance;
        let max_squared = self.max_distance * self.max_distance;
        // Fall back to hypot if squaring overflows or underflows.
        if distance_squared.is_finite()
            && (distance_squared > 0.0 || (delta.x == 0.0 && delta.y == 0.0))
            && (self.min_distance == 0.0 || min_squared > 0.0)
            && distance_squared >= min_squared
            && distance_squared <= max_squared
        {
            return;
        }

        let w1 = particles[self.p1].inverse_mass();
        let w2 = particles[self.p2].inverse_mass();
        let total_weight = w1 + w2;
        if total_weight == 0.0 {
            return;
        }

        let distance = delta.x.hypot(delta.y);
        let target = distance.clamp(self.min_distance, self.max_distance);
        if distance == target {
            return;
        }

        // Coincident points have no direction; choose a deterministic separation axis.
        let direction = if distance > 0.0 {
            delta / distance
        } else {
            Point2d::new(1.0, 0.0)
        };
        let error = distance - target;
        particles[self.p1].current.x += direction.x * error * (w1 / total_weight);
        particles[self.p1].current.y += direction.y * error * (w1 / total_weight);
        particles[self.p2].current.x -= direction.x * error * (w2 / total_weight);
        particles[self.p2].current.y -= direction.y * error * (w2 / total_weight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn particle(x: f64, mass: f64) -> Particle {
        Particle::new(x, 0.0, 0.0, 0.0, mass, 0.0, 0.0)
    }

    fn constraint(min_distance: f64, max_distance: f64) -> Constraint {
        Constraint {
            p1: 0,
            p2: 1,
            min_distance,
            max_distance,
        }
    }

    #[test]
    fn corrects_stretched_and_compressed_segments() {
        for distance in [2.0, 20.0] {
            let mut particles = [particle(0.0, 1.0), particle(distance, 1.0)];
            constraint(10.0, 10.0).fix(&mut particles);
            assert!((particles[1].current.x - particles[0].current.x - 10.0).abs() < 1e-10);
            assert!((particles[0].current.x + particles[1].current.x - distance).abs() < 1e-10);
        }
    }

    #[test]
    fn respects_distance_interval_including_endpoints() {
        for distance in [5.0, 7.0, 10.0] {
            let mut particles = [particle(0.0, 1.0), particle(distance, 1.0)];
            constraint(5.0, 10.0).fix(&mut particles);
            assert_eq!(particles[0].current.x, 0.0);
            assert_eq!(particles[1].current.x, distance);
        }
    }

    #[test]
    fn diagonal_distances_respect_interval_and_contact_threshold() {
        for (min, max) in [(5.0, 5.0), (4.0, 6.0), (2.0, f64::MAX)] {
            let mut particles = [particle(0.0, 1.0), particle(3.0, 1.0)];
            particles[1].current.y = 4.0;
            constraint(min, max).fix(&mut particles);
            assert_eq!(particles[0].current, Point2d::new(0.0, 0.0));
            assert_eq!(particles[1].current, Point2d::new(3.0, 4.0));
        }
    }

    #[test]
    fn diagonal_overlap_is_corrected() {
        let mut particles = [particle(0.0, 0.0), particle(3.0, 1.0)];
        particles[1].current.y = 4.0;
        constraint(10.0, f64::MAX).fix(&mut particles);
        assert_eq!(particles[1].current, Point2d::new(6.0, 8.0));
    }

    #[test]
    fn extreme_distances_fall_back_to_hypot() {
        for (distance, target) in [(1e200, 5e199), (1e-200, 5e-201)] {
            let mut particles = [particle(0.0, 0.0), particle(distance, 1.0)];
            constraint(target, target).fix(&mut particles);
            assert!((particles[1].current.x / target - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn distributes_correction_by_inverse_mass() {
        let mut particles = [particle(0.0, 1.0), particle(20.0, 3.0)];
        constraint(10.0, 10.0).fix(&mut particles);
        assert_eq!(particles[0].current.x, 7.5);
        assert_eq!(particles[1].current.x, 17.5);
    }

    #[test]
    fn preserves_anchors_at_either_end() {
        for masses in [[0.0, 1.0], [1.0, -1.0], [0.0, -1.0]] {
            let mut particles = [particle(0.0, masses[0]), particle(20.0, masses[1])];
            constraint(10.0, 10.0).fix(&mut particles);
            for i in 0..2 {
                if masses[i] <= 0.0 {
                    assert_eq!(particles[i].current.x, i as f64 * 20.0);
                }
            }
        }
    }

    #[test]
    fn separates_coincident_particles_without_nan() {
        let mut particles = [particle(0.0, 0.0), particle(0.0, 1.0)];
        constraint(10.0, 10.0).fix(&mut particles);
        assert_eq!(particles[0].current, Point2d::new(0.0, 0.0));
        assert_eq!(particles[1].current, Point2d::new(10.0, 0.0));
    }
}
