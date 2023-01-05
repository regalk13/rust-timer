extern crate sdl2;

use std::env;
use std::path::Path;
use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};

mod timer {
    #[derive(Copy, Clone, Default)]
    pub enum State {
        Paused,
        #[default]
        Running,
    }

    #[derive(Default)]
    pub struct Timer {
        state: State,
        pub seconds: u32,
    }

    impl Timer {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn get_time(&mut self) -> (u32, u32, u32) {
            let mut seconds = self.seconds;
            let hour = seconds / 3600;
            seconds = seconds % 3600;
            let minutes = seconds / 60;
            seconds = seconds % 60;
            (hour, minutes, seconds)
        }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    const DELTA_TIME: f64 = 1.0 / 60.0;

    let window = video_subsystem
        .window("Timer", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Rendering the timer with \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut timer = timer::Timer::new();

    timer.seconds = 12345;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // Event::KeyDown {
                //    keycode: Some(Keycode::Space),
                //    repeat: false,
                //    ..
                // } => {
                //     game.toggle_state();
                // }
                _ => {}
            }
        }
        let actual_time = timer.get_time();

        println!("{}:{}:{}", actual_time.0, actual_time.1, actual_time.2);
        timer.seconds += 1;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let seconds = time::Duration::from_secs((1.0 * 60.0 * DELTA_TIME) as u64);
        thread::sleep(seconds);
    }

    Ok(())
}
