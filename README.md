# The Rust Timer

<div align="center">
<img height="500" src="https://github.com/regalk13/rust-timer/blob/main/assets/timer-test.gif" />
</div>
<br>
Why not rustify the time world using SDL2? 

## Build

### Requirements

#### Rust

The latest stable release of Rust.

#### *SDL2.0 development libraries*

SDL2 >= 2.0.14 is recommended to use these bindings; below 2.0.14, you may experience link-time errors as some functions are used here but are not defined in SDL2. If you experience this issue because you are on a LTS machine (for instance, Ubuntu 12.04 or Ubuntu 14.04), we definitely recommend you to use the feature "bundled" which will compile the lastest stable version of SDL2 for your project.

## Usage

The timer brigns some options you can use with flags, if you're using `cargo run` remember ad `--` before the flag.

- Normal mode (Ascending) `./demo || cargo run
- Descending mode `./demo -d 25:13 || cargo run -- -d 25:13`
- Change the font (the default one is Roboto but you can change to your favorite font, search one [here](https://fonts.google.com/)) `./demo -f font.ttf|.fnt|...` || `cargo run -- -f font.ttf|.fnt|...`
- Press space to pause the timer, or esc to exit.

## Notes

If you're going to make a descending mode remember add the time in the format `hh:mm:ss`, 10 minutes != `10`. 10 minutes == `10:00`
