use std::fmt;
use std::mem;

pub type Zone = Vec<Vec<bool>>;

pub struct Figure {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    zone: Zone,
}

impl Figure {
    pub fn new(x: usize, y: usize, zone: Vec<Vec<bool>>) -> Self {
        Figure {
            x,
            y,
            zone: zone.clone(),
            width: zone.len(),
            height: zone[0].len(),
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn zone(&self) -> &Vec<Vec<bool>> {
        &self.zone
    }

    /*
        pub fn rotate_right(&mut self) {
            let mut temp_zone = vec![vec![false; self.width]; self.height];
            for j in 0..self.height {
                for i in 0..self.width {
                    temp_zone[self.height - j - 1][i] = self.zone[i][j];
                }
            }
            self.zone = temp_zone;
            mem::swap(&mut self.width, &mut self.height);
        }

        pub fn rotate_left(&mut self) {
            let mut temp_zone = vec![vec![false; self.width]; self.height];
            for j in 0..self.height {
                for i in 0..self.width {
                    temp_zone[j][self.width - i - 1] = self.zone[i][j];
                }
            }
            self.zone = temp_zone;
            mem::swap(&mut self.width, &mut self.height);
        }
    */
}

impl fmt::Debug for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::from("\r\n");
        for i in &self.zone {
            for j in i {
                string = string.to_owned() + if *j { "#" } else { " " };
            }
            string = string.to_owned() + "\r\n";
        }
        write!(f, "({}, {}){}", self.x, self.y, &string)
    }
}

pub fn create_figure(figure_type: i32) -> Figure {
    let x = 4_usize;
    let y = 0_usize;
    match figure_type {
        0 => Figure::new(x, y, vec![vec![true, true, true, true]]),
        1 => Figure::new(x, y, vec![vec![true, false, false], vec![true, true, true]]),
        2 => Figure::new(x, y, vec![vec![false, false, true], vec![true, true, true]]),
        3 => Figure::new(x, y, vec![vec![true, true], vec![true, true]]),
        4 => Figure::new(x, y, vec![vec![false, true, true], vec![true, true, false]]),
        5 => Figure::new(x, y, vec![vec![false, true, false], vec![true, true, true]]),
        6 => Figure::new(x, y, vec![vec![true, true, false], vec![false, true, true]]),
        _ => panic!("Wrong figure type"),
    }
}

pub fn move_right(figure: &Figure) -> Figure {
    Figure::new(figure.x() + 1, figure.y(), figure.zone().clone())
}

pub fn move_left(figure: &Figure) -> Figure {
    Figure::new(figure.x() - 1, figure.y(), figure.zone().clone())
}

pub fn rotate_right(figure: &Figure) -> Figure {
    let mut f = Figure::new(figure.x(), figure.y(), figure.zone().clone());
    let mut temp_zone = vec![vec![false; f.width()]; f.height()];
    for j in 0..f.height() {
        for i in 0..f.width() {
            temp_zone[f.height() - j - 1][i] = f.zone()[i][j];
        }
    }
    f.zone = temp_zone;
    mem::swap(&mut f.width, &mut f.height);
    f
}

pub fn rotate_left(figure: &Figure) -> Figure {
    let mut f = Figure::new(figure.x(), figure.y(), figure.zone().clone());
    let mut temp_zone = vec![vec![false; f.width]; f.height];
    for j in 0..f.height() {
        for i in 0..f.width() {
            temp_zone[j][f.width - i - 1] = f.zone()[i][j];
        }
    }
    f.zone = temp_zone;
    mem::swap(&mut f.width, &mut f.height);
    f
}
