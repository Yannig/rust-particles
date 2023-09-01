extern crate sdl2;
mod particle;
mod world;
mod constraint;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use crate::world::World;
use crate::particle::Particle;

fn create_rope(world: &mut World, start_x: f64, start_y: f64, end_x: f64, end_y: f64, node_count: u32) {
    let step_x = (end_x - start_x) / (node_count as f64);
    let step_y = (end_y - start_y) / (node_count as f64);
    let max_d = f64::sqrt(step_x * step_x + step_y * step_y);
    world.add_particle(Particle::new(start_x, start_y, 0.0, 0.0, -1.0, 1.0, 0.0));
    for i in 1..node_count {
        world.add_particle(Particle::new(start_x + (i as f64 * step_x), start_y + (i as f64 * step_y), 0.0, 0.0, 1.0, 1.0, 0.0));

    }
    // world.add_constraint(0.0, max_d);
    /* for i in 1.._particles.len() {
        // world.constraints.push(&Constraint { p1: &_particles[i - 1], p2: &_particles[i], min_distance: 0.0, max_distance: max_d });
    }*/
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut world = World::new(1800.0, 900.0);
    let window = video_subsystem
        .window("Graphisme 2D", world.x(), world.y())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut p = Particle::new(1.0, 10.0,10.0, 0.0, 1.0, 0.3, 0.2);
    world.add_particle(p);
    // world.particles.push(p);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    let mut p = Particle::new(1.0, 10.0,10.0, 0.0, 1.0, 0.3, 0.2);
                    world.add_particle(p);
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let x = (world.x() / 2) as f64;
                    create_rope(&mut world, x, 5.0,x + 250.0 , 500.0, 50);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        world.update();
        world.draw(&mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
