extern crate piston_window;

use piston_window::*;
use std::path::Path;

mod settings;
use crate::settings::*;
mod game;
use crate::game::*;
mod figure;
mod gamezone;
mod infozone;
mod level;

fn main() {
    let settings = Settings::new();
    let mut game = Game::new();
    let mut window: PistonWindow = WindowSettings::new(
        "Tetris v0.1.5 pre-alpha",
        [settings.game_width, settings.game_height],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut glyphs = window
        .load_font(Path::new("assets/fonts/ARCADECLASSIC.TTF"))
        .unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            game.button_press(button);
        };

        if let Some(button) = e.release_args() {
            game.button_release(button);
        };

        if let Some(args) = e.update_args() {
            game.update(args);
        };

        if let Some(args) = e.resize_args() {
            game.resize(args);
        };

        game.render(&mut window, &e, &mut glyphs);
    }
}
