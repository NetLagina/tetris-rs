extern crate piston_window;

use piston_window::*;

use crate::gamezone::*;
use crate::infozone::*;
use crate::level::Level;
use crate::settings::Settings;

const COLOR_WHITE: [f32; 4] = [1.0; 4];

const FIGURE_SIDE_PX: f64 = 20_f64;

pub struct Game {
    info_zone: InfoZone,
    game_zone: GameZone,
    level: Level,
    prev_time: f64,
    score: u32,
    game_over: bool,
    settings: Settings,
    render_zoom_coefficient: f64,
    is_step_pressed: bool,
    step_prev_time: f64,
}

impl Game {
    pub fn new() -> Self {
        let settings = Settings::new();
        Game {
            info_zone: InfoZone::new(
                INFO_ZONE_X,
                INFO_ZONE_Y,
                1_f64,
                COLOR_WHITE,
                INFO_ZONE_FONT_SIZE,
            ),
            game_zone: GameZone::new(
                GAME_ZONE_X,
                GAME_ZONE_Y,
                GAME_ZONE_WIDTH,
                GAME_ZONE_HEIGHT,
                1_f64,
            ),
            level: Level::new(),
            prev_time: 0_f64,
            score: 0,
            game_over: false,
            settings,
            render_zoom_coefficient: 1_f64,
            is_step_pressed: false,
            step_prev_time: 0_f64,
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
            self.level.step(&mut self.score);
            self.is_step_pressed = true;
        }
    }

    pub fn button_release(&mut self, button: Button) {
        if button == Button::Keyboard(Key::S) {
            self.is_step_pressed = false;
            self.step_prev_time = 0_f64;
        }
    }

    pub fn resize(&mut self, args: ResizeArgs) {
        let draw_size = args.draw_size;
        self.render_zoom_coefficient = if (draw_size[0] as f64 / self.settings.game_width as f64)
            < (draw_size[1] as f64 / self.settings.game_height as f64)
        {
            draw_size[0] as f64 / self.settings.game_width as f64
        } else {
            draw_size[1] as f64 / self.settings.game_height as f64
        };
        self.info_zone = InfoZone::new(
            INFO_ZONE_X,
            INFO_ZONE_Y,
            self.render_zoom_coefficient,
            COLOR_WHITE,
            INFO_ZONE_FONT_SIZE,
        );
        self.game_zone = GameZone::new(
            GAME_ZONE_X,
            GAME_ZONE_Y,
            GAME_ZONE_WIDTH,
            GAME_ZONE_HEIGHT,
            self.render_zoom_coefficient,
        );
    }

    pub fn render(&self, window: &mut PistonWindow, e: &Event, glyphs: &mut Glyphs) {
        let width = window.draw_size().width;
        let height = window.draw_size().height;
        window.draw_2d(e, |c, g, device| {
            clear([0.0; 4], g);

            /* window 640x480 */
            line(COLOR_WHITE, 1.0, [0.0, 0.0, width, 0.0], c.transform, g);
            line(
                COLOR_WHITE,
                1.0,
                [width, 0.0, width, height],
                c.transform,
                g,
            );
            line(
                COLOR_WHITE,
                1.0,
                [width, height, 0.0, height],
                c.transform,
                g,
            );
            line(COLOR_WHITE, 1.0, [0.0, 0.0, 0.0, height], c.transform, g);
            /* window end */

            if !self.game_over {
                /* info zone */
                let text = format!("Score {}", self.score);
                self.info_zone
                    .text()
                    .draw(
                        text.as_str(),
                        glyphs,
                        &c.draw_state,
                        c.trans(self.info_zone.x(), self.info_zone.y()).transform,
                        g,
                    )
                    .unwrap();
                glyphs.factory.encoder.flush(device);
                /* info zone end */

                /* game field */
                line(
                    COLOR_WHITE,
                    1.0,
                    [
                        self.game_zone.x(),
                        self.game_zone.y(),
                        self.game_zone.x() + self.game_zone.width(),
                        self.game_zone.y(),
                    ],
                    c.transform,
                    g,
                );
                line(
                    COLOR_WHITE,
                    1.0,
                    [
                        self.game_zone.x() + self.game_zone.width(),
                        self.game_zone.y(),
                        self.game_zone.x() + self.game_zone.width(),
                        self.game_zone.y() + self.game_zone.height(),
                    ],
                    c.transform,
                    g,
                );
                line(
                    COLOR_WHITE,
                    1.0,
                    [
                        self.game_zone.x() + self.game_zone.width(),
                        self.game_zone.y() + self.game_zone.height(),
                        self.game_zone.x(),
                        self.game_zone.y() + self.game_zone.height(),
                    ],
                    c.transform,
                    g,
                );
                line(
                    COLOR_WHITE,
                    1.0,
                    [
                        self.game_zone.x(),
                        self.game_zone.y(),
                        self.game_zone.x(),
                        self.game_zone.y() + self.game_zone.height(),
                    ],
                    c.transform,
                    g,
                );

                let square_side = FIGURE_SIDE_PX * self.render_zoom_coefficient; //px
                for i in 0..self.level.width() {
                    for j in 0..self.level.height() {
                        let x2 = i as f64 * square_side + self.game_zone.x() + 1.0;
                        let y2 = j as f64 * square_side + self.game_zone.y() + 1.0;
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
                    COLOR_WHITE,
                    [
                        self.game_zone.x(),
                        self.game_zone.y(),
                        self.game_zone.width(),
                        self.game_zone.height(),
                    ],
                    c.transform,
                    g,
                );
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        const STEP_TIME: f64 = 1.0;
        self.prev_time += args.dt;
        if self.prev_time > STEP_TIME {
            if self.level.step(&mut self.score) {
                self.prev_time = 0.0;
            } else {
                self.game_over = true;
            }
        }

        const AUTO_STEP_TIME: f64 = 0.05;
        if self.is_step_pressed {
            self.step_prev_time += args.dt;
            if self.step_prev_time > AUTO_STEP_TIME {
                self.level.step(&mut self.score);
                self.step_prev_time = 0.0;
            }
        }
    }
}
