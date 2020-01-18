use std::fmt;
use std::mem;

pub type Zone = Vec<Vec<bool>>;

pub struct Figure {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: [f32; 4],
    zone: Zone,
}

impl Figure {
    pub fn new(x: usize, y: usize, zone: Vec<Vec<bool>>, color: [f32; 4]) -> Self {
        Figure {
            x,
            y,
            color,
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

    pub fn color(&self) -> &[f32; 4] {
        &self.color
    }
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

pub fn create_figure(figure_type: i32, figure_color: [f32; 4]) -> Figure {
    let x = 4_usize;
    let y = 0_usize;
    match figure_type {
        0 => Figure::new(x, y, vec![vec![true, true, true, true]], figure_color),
        1 => Figure::new(
            x,
            y,
            vec![vec![true, false, false], vec![true, true, true]],
            figure_color,
        ),
        2 => Figure::new(
            x,
            y,
            vec![vec![false, false, true], vec![true, true, true]],
            figure_color,
        ),
        3 => Figure::new(x, y, vec![vec![true, true], vec![true, true]], figure_color),
        4 => Figure::new(
            x,
            y,
            vec![vec![false, true, true], vec![true, true, false]],
            figure_color,
        ),
        5 => Figure::new(
            x,
            y,
            vec![vec![false, true, false], vec![true, true, true]],
            figure_color,
        ),
        6 => Figure::new(
            x,
            y,
            vec![vec![true, true, false], vec![false, true, true]],
            figure_color,
        ),
        _ => panic!("Wrong figure type"),
    }
}

pub fn move_right(figure: &Figure) -> Figure {
    Figure::new(
        figure.x() + 1,
        figure.y(),
        figure.zone().clone(),
        *figure.color(),
    )
}

pub fn move_left(figure: &Figure) -> Figure {
    Figure::new(
        figure.x() - 1,
        figure.y(),
        figure.zone().clone(),
        *figure.color(),
    )
}

pub fn rotate_right(figure: &Figure) -> Figure {
    let mut f = Figure::new(
        figure.x(),
        figure.y(),
        figure.zone().clone(),
        *figure.color(),
    );
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
    let mut f = Figure::new(
        figure.x(),
        figure.y(),
        figure.zone().clone(),
        *figure.color(),
    );
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
