extern crate piston_window;

use piston_window::*;

mod settings;
use crate::settings::*;
mod game;
use crate::game::*;
mod figure;
mod gamezone;
mod level;

fn main() {
    let settings = Settings::new();
    let mut game = Game::new();
    let mut window: PistonWindow = WindowSettings::new(
        "Tetris v0.1.2 pre-alpha",
        [settings.game_width, settings.game_height],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            game.button_press(button);
        };

        if let Some(args) = e.update_args() {
            game.update(args);
        };

        if let Some(args) = e.resize_args() {
            game.resize(args);
        };

        game.render(&mut window, &e);
    }
}
