extern crate sdl2;

use std::path::Path;
use std::{env, num::ParseIntError, thread, time};

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
    use crate::ParseIntError;
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

    pub struct ColorConverter {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl ColorConverter {
        pub fn new(hex_code: &str) -> Result<Self, String> {
            if hex_code.is_empty() {
                return Err("ERROR: Empty hex color".to_string());
            }

            let hex_code = if hex_code.starts_with('#') {
                crop_letters(hex_code, 1)
            } else {
                hex_code
            };

            let hex_code = if hex_code.len() == 3 {
                repeat_letters(hex_code, 1)
            } else {
                hex_code.to_owned()
            };
            if hex_code.len() % 2 != 0 {
                return Err("ERROR: Invalid hex color".to_string());
            }

            let decoded_values = decode_hex(&hex_code).unwrap_or_default();
            if decoded_values.is_empty() || decoded_values.len() > 4 {
                return Err("ERROR: Invalid hex color".to_string());
            }

            let color = Self {
                red: decoded_values[0],
                green: decoded_values[1],
                blue: decoded_values[2],
            };

            Ok(color)
        }
    }

    fn crop_letters(s: &str, pos: usize) -> &str {
        match s.char_indices().nth(pos) {
            Some((pos, _)) => &s[pos..],
            None => "",
        }
    }

    fn repeat_letters(s: &str, repetitions: i32) -> String {
        let mut output = String::from("");
        for char in s.chars() {
            for _ in 0..=repetitions {
                output.push(char);
            }
        }

        output
    }
    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
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

pub fn run(path: &Path, time: u32, countdown: bool, exit: bool) -> Result<(), String> {
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
    let mut bcolor = (0, 0, 0);
    let mut fcolor = (255, 255, 255);

    // Getting colors
    match env::var("TIMER_BACKGROUND") {
        Ok(val) => {
            let rgb = timer::ColorConverter::new(&val);
            match rgb {
                Ok(c) => bcolor = (c.red, c.green, c.blue),
                Err(e) => {
                    println!("Error trying to get background color");
                    println!("{}", e);
                }
            }
        }
        Err(_) => {
            canvas.set_draw_color(Color::RGB(bcolor.0, bcolor.1, bcolor.2));
        }
    }

    match env::var("TIMER_FOREGROUND") {
        Ok(val) => {
            let rgb = timer::ColorConverter::new(&val);
            match rgb {
                Ok(c) => fcolor = (c.red, c.green, c.blue),
                Err(e) => {
                    println!("Error trying to get background color");
                    println!("{}", e);
                }
            }
        }
        Err(_) => {
            canvas.set_draw_color(Color::RGB(fcolor.0, fcolor.1, fcolor.2));
        }
    }

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut timer = timer::Timer::new();

    timer.seconds = time;

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
            if countdown {
                if !(timer.seconds <= 0) {
                    timer.seconds -= 1;
                } else if timer.seconds == 0 && exit {
                    break 'running;
                }
            } else {
                timer.seconds += 1;
            }

            canvas.set_draw_color(Color::RGB(bcolor.0, bcolor.1, bcolor.2));
            canvas.clear();

            let seconds = time::Duration::from_secs((1.0 * 60.0 * DELTA_TIME) as u64);

            let mut font = ttf_context.load_font(path, 128)?;
            font.set_style(sdl2::ttf::FontStyle::NORMAL);
            let surface = font
                .render(&format!(
                    "{}:{}:{}",
                    actual_time.0, actual_time.1, actual_time.2
                ))
                .blended(Color::RGB(fcolor.0, fcolor.1, fcolor.2))
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
    let mut countdown: bool = false;
    let mut exit_after_end: bool = false;
    let mut time = 0;

    let font = env::var("TIMER_FONT_FAMILY");
    let mut value = String::new();
    match font {
        Ok(val) => {
            value = val;
        }
        Err(_) => {
            println!("You need to set a font");
            println!("export TIMER_FONT_FAMILY=~/fontpath");
            return;
        }
    }

    let mut path = Path::new(&value);

    if args.len() < 2 {
        println!("Usage: ./timer -help --h")
    } else {
        for (i, arg) in args.iter().enumerate() {
            if arg == "-h" || arg == "--help" {
                println!("-- Welcome to the help match --");
                println!("match flags {{ \n    -d [time] => countdown specified time.\n    -de [time] => countdown specified time then exit. \n    -f [font] => use a different font at the specified in env var. \n }}");
                return;
            }
            if arg == "-d" {
                countdown = true;
                if !(i >= args.len() - 1) {
                    time = parse_to_seconds(args[i + 1].clone());
                }
            } else if arg == "-de" {
                countdown = true;
                exit_after_end = true;
                if !(i >= args.len() - 1) {
                    time = parse_to_seconds(args[i + 1].clone());
                }
            } else if arg == "-f" {
                if !(i >= args.len() - 1) {
                    path = Path::new(&args[i + 1]);
                }
            }
        }
    }

    run(path, time, countdown, exit_after_end);
}
