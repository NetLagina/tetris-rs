extern crate piston_window;

use piston_window::*;

use crate::level::Level;

pub struct Game {
    game_zone: GameZone,
    level: Level,
    prev_time: f64,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            game_zone: GameZone::new(10.0, 30.0, 202.0, 402.0),
            level: Level::new(),
            prev_time: 0_f64,
            game_over: false,
        }
    }

    pub fn button_press(&mut self, button: Button) {
        if button == Button::Keyboard(Key::D) {
            self.level.move_right();
        }
        if button == Button::Keyboard(Key::A) {
            self.level.move_left();
        }
        if button == Button::Keyboard(Key::E) {
            self.level.rotate_right();
        }
        if button == Button::Keyboard(Key::Q) {
            self.level.rotate_left();
        }
        if button == Button::Keyboard(Key::S) {
            self.level.step();
        }
    }

    pub fn render(&self, window: &mut PistonWindow, e: &Event) {
        let window_settings = window.window.size();
        window.draw_2d(e, |c, g, _device| {
            clear([0.0; 4], g);

            /* window 640x480 */
            line(
                [1.0; 4],
                1.0,
                [0.0, 0.0, window_settings.width, 0.0],
                c.transform,
                g,
            );
            line(
                [1.0; 4],
                1.0,
                [
                    window_settings.width,
                    0.0,
                    window_settings.width,
                    window_settings.height,
                ],
                c.transform,
                g,
            );
            line(
                [1.0; 4],
                1.0,
                [
                    window_settings.width,
                    window_settings.height,
                    0.0,
                    window_settings.height,
                ],
                c.transform,
                g,
            );
            line(
                [1.0; 4],
                1.0,
                [0.0, 0.0, 0.0, window_settings.height],
                c.transform,
                g,
            );
            /* window end */

            if !self.game_over {
                /* game field */
                line(
                    [1.0; 4],
                    1.0,
                    [
                        self.game_zone.x,
                        self.game_zone.y,
                        self.game_zone.x + self.game_zone.width,
                        self.game_zone.y,
                    ],
                    c.transform,
                    g,
                );
                line(
                    [1.0; 4],
                    1.0,
                    [
                        self.game_zone.x + self.game_zone.width,
                        self.game_zone.y,
                        self.game_zone.x + self.game_zone.width,
                        self.game_zone.y + self.game_zone.height,
                    ],
                    c.transform,
                    g,
                );
                line(
                    [1.0; 4],
                    1.0,
                    [
                        self.game_zone.x + self.game_zone.width,
                        self.game_zone.y + self.game_zone.height,
                        self.game_zone.x,
                        self.game_zone.y + self.game_zone.height,
                    ],
                    c.transform,
                    g,
                );
                line(
                    [1.0; 4],
                    1.0,
                    [
                        self.game_zone.x,
                        self.game_zone.y,
                        self.game_zone.x,
                        self.game_zone.y + self.game_zone.height,
                    ],
                    c.transform,
                    g,
                );

                let square_side = 20_f64; //px
                for i in 0..self.level.width() {
                    for j in 0..self.level.height() {
                        let x2 = i as f64 * square_side + self.game_zone.x + 1.0;
                        let y2 = j as f64 * square_side + self.game_zone.y + 1.0;
                        if self.level.get(i, j) {
                            rectangle(
                                [0.86, 0.86, 0.86, 1.0],
                                [x2, y2, square_side, square_side],
                                c.transform,
                                g,
                            );
                        }
                        if self.level.get_figure(i, j) {
                            rectangle(
                                *self.level.figure().unwrap().color(),
                                [x2, y2, square_side, square_side],
                                c.transform,
                                g,
                            );
                        }
                    }
                }
            /* game field end */
            } else {
                rectangle(
                    [1.0; 4],
                    [
                        self.game_zone.x,
                        self.game_zone.y,
                        self.game_zone.width,
                        self.game_zone.height,
                    ],
                    c.transform,
                    g,
                );
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.prev_time += args.dt;
        if self.prev_time > 1.0 {
            if self.level.step() {
                self.prev_time = 0.0;
            } else {
                self.game_over = true;
            }
        }
    }
}

struct GameZone {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl GameZone {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        GameZone {
            x,
            y,
            width,
            height,
        }
    }
}
