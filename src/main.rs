extern crate sdl2;
mod particle;
mod world;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use crate::world::World;

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

    world.create_particle(1.0, 10.0,10.0, 0.0, 1.0, 0.3, 0.2);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    world.create_particle(1.0, 10.0,10.0, 0.0, 1.0, 0.3, 0.1);
                }
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    world.create_particle((world.x() / 2) as f64, 1.0,0.0, 0.0, -1.0, 0.3, 0.1);
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
