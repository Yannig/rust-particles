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
use sdl2::rect::Point;
use std::time::{Duration, Instant};

fn add_demo_particle(world: &mut World) {
    let radius = rand::random_range(5.0..=60.0);
    let start = 20.0_f64.max(radius + 2.0);
    let mass = 10.0 * (radius / 12.0).powi(2);
    let color = Color::RGB(
        rand::random_range(30..=200),
        rand::random_range(30..=200),
        rand::random_range(30..=200),
    );
    world.add_particle(
        Particle::new(start, start, 10.0, 0.0, mass, 0.85, 0.2)
            .with_radius(radius)
            .with_color(color),
    );
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
            "Verlet | R : corde | F : drapeau | N : particule | E : explosion | Echap : quitter",
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
    let mut explosion_flash = None;
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
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    repeat: false,
                    ..
                } => {
                    let center = Point2d::new(
                        rand::random_range(0.1..=0.9) * world.x() as f64,
                        rand::random_range(0.92..=0.98) * world.y() as f64,
                    );
                    let radius = rand::random_range(300.0..=450.0);
                    world.explode(center, radius, rand::random_range(400.0..=700.0));
                    explosion_flash = Some((center, radius, Instant::now()));
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
        if let Some((center, radius, started)) = explosion_flash {
            let progress = started.elapsed().as_secs_f64() / 0.4;
            if progress < 1.0 {
                canvas.set_draw_color(Color::RGB(240, 130, 20));
                let point = |angle: f64| {
                    Point::new(
                        (center.x + angle.cos() * radius * progress) as i32,
                        (center.y + angle.sin() * radius * progress) as i32,
                    )
                };
                for i in 0..64 {
                    let angle = std::f64::consts::TAU / 64.0;
                    canvas.draw_line(point(i as f64 * angle), point((i + 1) as f64 * angle))?;
                }
            } else {
                explosion_flash = None;
            }
        }
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
