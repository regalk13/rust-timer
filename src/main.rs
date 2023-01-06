extern crate sdl2;

use std::env;
use std::path::Path;
use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::TextureQuery;

use regex::Regex;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
mod timer {
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub enum State {
        Paused,
        #[default]
        Running,
    }

    #[derive(Default)]
    pub struct Timer {
        pub state: State,
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
        pub fn toggle_state(&mut self) {
            self.state = match self.state {
                State::Paused => State::Running,
                State::Running => State::Paused,
            }
        }
    }
}
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn run(path: &Path) -> Result<(), String> {
    // init context
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    const DELTA_TIME: f64 = 1.0 / 60.0;

    let window = video_subsystem
        .window("Timer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

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
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    timer.toggle_state();
                }
                _ => {}
            }
        }
        if !(timer.state == timer::State::Paused) {
            let actual_time = timer.get_time();
            timer.seconds += 1;

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let seconds = time::Duration::from_secs((1.0 * 60.0 * DELTA_TIME) as u64);

            let mut font = ttf_context.load_font(path, 128)?;
            font.set_style(sdl2::ttf::FontStyle::NORMAL);
            let surface = font
                .render(&format!(
                    "{}:{}:{}",
                    actual_time.0, actual_time.1, actual_time.2
                ))
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            let TextureQuery { width, height, .. } = texture.query();
            // If the example text is too big for the screen, downscale it (and center irregardless)
            let padding = 64;
            let target = get_centered_rect(
                width,
                height,
                SCREEN_WIDTH - padding,
                SCREEN_HEIGHT - padding,
            );
            canvas.copy(&texture, None, Some(target))?;
            canvas.present();
            thread::sleep(seconds);
        }
    }

    Ok(())
}

fn parse_to_seconds(time: String) -> u32 {
    // Format example 5:25:13
    let re = Regex::new(r"^(?:(?:([01]?\d|2[0-3]):)?([0-5]?\d):)?([0-5]?\d)$").unwrap(); 
    let caps = re.captures(&time).unwrap();
    let mut seconds = 0;

    if let Some(hours) = caps.get(1) {
        seconds += hours.as_str().parse::<u32>().unwrap() * 3600;
    }
    
    if let Some(minutes) = caps.get(2) {
        seconds += minutes.as_str().parse::<u32>().unwrap() * 60; 
    }

    if let Some(seconds_) = caps.get(3) {
        seconds += seconds_.as_str().parse::<u32>().unwrap(); 
    }
    if seconds == 0 {
        println!("Time parsed to 0! if it's intensional ignore this");
    }
    seconds
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let mut path: &Path = Path::new("");
    let mut countdown: bool = false;
    let mut time = 0;
    if args.len() < 2 {
        println!("Usage: ./demo -help --h")
    } else {
        for (i, arg) in args.iter().enumerate() { 
            if arg == "-d" {
                countdown = true;
                if !(i >= args.len()-1) {
                    time = parse_to_seconds(args[i+1].clone()); 
                }
            } else if arg == "-f" {
                if !(i >= args.len()-1) {
                    path = Path::new(&args[i+1]);
                } 
            }
        }
    }
    println!("Time: {}, coutndown: {}", time, countdown);
    run(path);
}
