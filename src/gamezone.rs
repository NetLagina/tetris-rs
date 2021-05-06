pub const GAME_ZONE_X: f64 = 10.0;
pub const GAME_ZONE_Y: f64 = 30.0;
pub const GAME_ZONE_WIDTH: f64 = 202.0;
pub const GAME_ZONE_HEIGHT: f64 = 402.0;

pub struct GameZone {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl GameZone {
    pub fn new(x: f64, y: f64, width: f64, height: f64, render_zoom_coefficient: f64) -> Self {
        GameZone {
            x: x * render_zoom_coefficient,
            y: y * render_zoom_coefficient,
            width: width * render_zoom_coefficient,
            height: height * render_zoom_coefficient,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}
