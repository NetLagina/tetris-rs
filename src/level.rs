use rand::prelude::*;
use std::fmt;

use crate::figure::*;

const ZONE_WIDTH: usize = 10;
const ZONE_HEIGHT: usize = 20;

pub struct Level {
    zone: Zone,
    figure: Option<Figure>,
}

impl Level {
    pub fn new() -> Self {
        let zone_t = vec![vec![false; ZONE_HEIGHT]; ZONE_WIDTH];
        Level {
            zone: zone_t,
            figure: None,
        }
    }

    pub fn width(&self) -> usize {
        self.zone.len()
    }

    pub fn height(&self) -> usize {
        (*self.zone.get(0).unwrap()).len()
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        *self.zone.get(i).unwrap().get(j).unwrap()
    }

    pub fn get_figure(&self, i: usize, j: usize) -> bool {
        match &self.figure {
            Some(f) => {
                if f.x() <= i && i < f.x() + f.width() && f.y() <= j && j < f.y() + f.height() {
                    f.zone()[i - f.x()][j - f.y()]
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn figure(&self) -> Option<&Figure> {
        self.figure.as_ref()
    }

    pub fn step(&mut self) -> bool {
        let game_over;
        match &mut self.figure {
            Some(f) => {
                let figure = Figure::new(f.x(), f.y() + 1, f.zone().clone(), *f.color());
                if (figure.y() + figure.height() - 1) < ZONE_HEIGHT {
                    if is_free_space(&self.zone, &figure) {
                        f.set_y(figure.y());
                    } else {
                        for i in 0..f.width() {
                            for j in 0..f.height() {
                                if f.zone()[i][j] {
                                    self.zone[f.x() + i][f.y() + j] = true;
                                }
                            }
                        }
                        self.figure = None;
                    }
                } else {
                    for i in 0..f.width() {
                        for j in 0..f.height() {
                            if f.zone()[i][j] {
                                self.zone[f.x() + i][f.y() + j] = true;
                            }
                        }
                    }
                    self.figure = None;
                }
                game_over = false;
            }
            None => {
                for i in (0..ZONE_HEIGHT).rev() {
                    let mut full_row = true;
                    for j in 0..ZONE_WIDTH {
                        if !self.zone[j][i] {
                            full_row = false;
                        }
                    }
                    if full_row {
                        for i2 in (1..=i).rev() {
                            for j2 in 0..ZONE_WIDTH {
                                self.zone[j2][i2] = self.zone[j2][i2 - 1];
                            }
                        }
                    }
                }
                let mut rng = rand::thread_rng();
                let red = rng.gen_range(0, 2) as f32;
                let green = rng.gen_range(0, 2) as f32;
                let blue = if red == 0.0 && green == 0.0 {
                    1.0
                } else {
                    rng.gen_range(0, 2) as f32
                };
                let figure = create_figure(rng.gen_range(0, 7), [red, green, blue, 1.0]);
                if !is_free_space(&self.zone, &figure) {
                    game_over = true;
                } else {
                    self.figure = Some(figure);
                    game_over = false;
                }
            }
        }
        !game_over
    }

    pub fn move_right(&mut self) {
        if let Some(f) = &mut self.figure {
            if f.x() + f.width() < ZONE_WIDTH {
                let fig = move_right(f);
                if is_free_space(&self.zone, &fig) {
                    f.set_x(fig.x());
                }
            }
        }
    }

    pub fn move_left(&mut self) {
        if let Some(f) = &mut self.figure {
            if 0 < f.x() {
                let fig = move_left(f);
                if is_free_space(&self.zone, &fig) {
                    f.set_x(fig.x());
                }
            }
        }
    }

    pub fn rotate_right(&mut self) {
        if let Some(f) = &mut self.figure {
            let fig = rotate_right(f);
            if is_free_space(&self.zone, &fig) {
                *f = fig;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        if let Some(f) = &mut self.figure {
            let fig = rotate_left(f);
            if is_free_space(&self.zone, &fig) {
                *f = fig;
            }
        }
    }
}

pub fn is_free_space(zone: &Zone, f: &Figure) -> bool {
    if (f.x() + f.width() - 1) < ZONE_WIDTH && (f.y() + f.height() - 1) < ZONE_HEIGHT {
        let mut is_cross = false;
        for i in 0..f.width() {
            for j in 0..f.height() {
                if f.zone()[i][j] && zone[f.x() + i][f.y() + j] {
                    is_cross = true;
                }
            }
        }
        !is_cross
    } else {
        false
    }
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::from("\r\n");
        for i in &self.zone {
            for j in i {
                string = string.to_owned() + if *j { "#" } else { " " };
            }
            string = string.to_owned() + "\r\n";
        }
        write!(f, "{}", &string)
    }
}
