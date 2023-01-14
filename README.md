<h1 align="center">The Rust Timer</h1>

 
<div align="center">
<img height="450" src="https://github.com/regalk13/rust-timer/blob/main/assets/timer-test.gif" />
</div>
<br>

Why not rustify the time world using SDL2? 

## Build

### Requirements

#### Rust

The latest stable release of Rust.

#### *SDL2.0 development libraries*

SDL2 >= 2.0.14 is recommended to use these bindings; below 2.0.14, you may experience link-time errors as some functions are used here but are not defined in SDL2. If you experience this issue because you are on a LTS machine (for instance, Ubuntu 12.04 or Ubuntu 14.04), we definitely recommend you to use the feature "bundled" which will compile the lastest stable version of SDL2 for your project.

For use SDL2 in Rust I'm using this [bindings](https://github.com/Rust-SDL2/rust-sdl2).

- Arch Linux
`# pacman -S sdl2 sdl2_ttf` 


## Usage

The timer brigns some options you can use with flags, if you're using `cargo run` remember ad `--` before the flag.

- Normal mode (Ascending) `./demo || cargo run
- Descending mode `./demo -d 25:13 || cargo run -- -d 25:13` (exit when ends `./demo -de 25:13 || cargo run -- -de 25:13`)
- Change the font (the default one is Roboto but you can change to your favorite font, search one [here](https://fonts.google.com/)) `./demo -f font.ttf|.fnt|...` || `cargo run -- -f font.ttf|.fnt|...`
- Clock mode `./demo -c || cargo run -- -c` get the current time on your machine.
- Press space to pause the timer, or esc to exit.

## Pomodoro

This timer can be used as your pomodoro timer in your linux machine!, example of a configuration in your shell:
```zsh

alias work="./timer -de 50:00 && notify-send 'PomodoroTimer' 'Your <b>work</b> time is up, take a break'"
alias break="./timer -de 10:00 && notify-send 'PomodoroTimer' 'Your <b>break</b> time is up, get back to work'"
```

## Customize

The timer have options that can be changed with enviroment variables:
```
TIMER_FONT_FAMILY = font_file
TIMER_BACKGROUND = hex_color
TIMER_FOREGROUND = hex_color
```
You can change it in your shell configuration or by exporting the variable (ephemeral):
```zsh
export TIMER_BACKGROUND="#0F0F0F"
```
## Notes

If you're going to make a descending mode remember add the time in the format `hh:mm:ss`, 10 minutes != `10`. 10 minutes == `10:00`

## todo!()

- [x] Customization. 
- [ ] Make an UI.
- [x] Help menu.
- [ ] Modulation of code.
- [x] Add some modes (pomodoro, clock, etc...).
