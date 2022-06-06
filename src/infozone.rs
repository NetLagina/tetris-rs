use crate::Text;
use piston_window::types::{Color, FontSize};

pub const INFO_ZONE_X: f64 = 240.0;
pub const INFO_ZONE_Y: f64 = 90.0;
pub const INFO_ZONE_FONT_SIZE: FontSize = 36;

pub struct InfoZone {
    x: f64,
    y: f64,
    text: Text,
}

impl InfoZone {
    pub fn new(
        x: f64,
        y: f64,
        render_zoom_coefficient: f64,
        text_color: Color,
        font_size: FontSize,
    ) -> Self {
        InfoZone {
            x: x * render_zoom_coefficient,
            y: y * render_zoom_coefficient,
            text: Text::new_color(text_color, font_size),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn text(&self) -> &Text {
        &self.text
    }
}
