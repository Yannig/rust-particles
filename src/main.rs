mod constraint;
mod particle;
mod point2d;
mod world;

use crate::particle::Particle;
use crate::point2d::Point2d;
use crate::world::{World, TIME_STEP};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn add_demo_particle(world: &mut World) {
    let radius = rand::random_range(5.0..=60.0);
    let start = 20.0_f64.max(radius + 2.0);
    let mass = 10.0 * (radius / 12.0).powi(2);
    world.add_particle(Particle::new(start, start, 10.0, 0.0, mass, 0.3, 0.2).with_radius(radius));
}

fn add_demo_rope(world: &mut World) {
    let x = world.x() as f64 / 2.0;
    world.create_rope(Point2d::new(x, 50.0), Point2d::new(x + 600.0, 250.0), 61);
}

fn add_demo_flag(world: &mut World) {
    let center_x = world.x() as f64 / 2.0;
    let anchor_y = world.y() as f64 / 2.0;
    world.create_flag(
        Point2d::new(center_x - 180.0, anchor_y - 240.0),
        Point2d::new(center_x + 180.0, anchor_y),
        13,
        9,
    );
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut world = World::new(1800.0, 900.0);
    let window = video_subsystem
        .window(
            "Verlet | R : corde | F : drapeau | N : particule | Echap : quitter",
            world.x(),
            world.y(),
        )
        .position_centered()
        .build()
        .map_err(|error| error.to_string())?;
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|error| error.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    add_demo_particle(&mut world);
    add_demo_rope(&mut world);
    add_demo_flag(&mut world);

    let mut previous_frame = Instant::now();
    let mut accumulator = 0.0;
    'running: loop {
        let frame_start = Instant::now();
        // Limit catch-up after a long pause to avoid a spiral of death.
        accumulator += frame_start
            .duration_since(previous_frame)
            .as_secs_f64()
            .min(0.25);
        previous_frame = frame_start;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    repeat: false,
                    ..
                } => {
                    add_demo_particle(&mut world);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } => {
                    add_demo_rope(&mut world);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    repeat: false,
                    ..
                } => {
                    add_demo_flag(&mut world);
                }
                _ => {}
            }
        }
        while accumulator >= TIME_STEP {
            world.update();
            accumulator -= TIME_STEP;
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        world.draw(&mut canvas)?;
        canvas.present();
        std::thread::sleep(
            Duration::from_secs_f64(TIME_STEP).saturating_sub(frame_start.elapsed()),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_particles_have_bounded_random_radii_and_start_inside_world() {
        let mut world = World::new(1800.0, 900.0);
        for _ in 0..128 {
            add_demo_particle(&mut world);
        }
        for p in &world.particles {
            assert!((5.0..=60.0).contains(&p.radius));
            assert!(p.current.x - p.radius >= 0.0);
            assert!(p.current.y - p.radius >= 0.0);
            assert!(p.current.x + p.radius <= world.max.x);
            assert!(p.current.y + p.radius <= world.max.y);
            assert_eq!(p.current - p.previous, Point2d::new(10.0, 0.0));
            assert!((p.mass / (p.radius * p.radius) - 10.0 / 144.0).abs() < 1e-12);
        }
    }
}
