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

fn add_demo_rope(world: &mut World) {
    let x = world.x() as f64 / 2.0;
    world.create_rope(Point2d::new(x, 50.0), Point2d::new(x + 600.0, 250.0), 61);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut world = World::new(1800.0, 900.0);
    let window = video_subsystem
        .window(
            "Corde Verlet | R : ajouter une corde | N : particule | Echap : quitter",
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
    world.add_particle(Particle::new(20.0, 20.0, 10.0, 0.0, 10.0, 0.3, 0.2).with_radius(12.0));
    add_demo_rope(&mut world);

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
                    world.add_particle(
                        Particle::new(20.0, 20.0, 10.0, 0.0, 10.0, 0.3, 0.2).with_radius(12.0),
                    );
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } => {
                    add_demo_rope(&mut world);
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
