extern crate piston_window;

use piston_window::*;

use crate::level::Level;

pub struct Game {
	level: Level,
	prev_time: f64,
}


impl Game {
	
	pub fn new() -> Self {
		Game {
			level: Level::new(),
			prev_time: 0_f64,
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
			self.level.rotate();
		}
		if button == Button::Keyboard(Key::Q) {
			self.level.rotate();
        }
		if button == Button::Keyboard(Key::S) {
			self.level.step();
        }
	}
	
	pub fn render(&self, window: &mut PistonWindow, e: piston_window::Event) {
		window.draw_2d(&e, |c, g, _device| {
            clear([0.0; 4], g);
			
			/* window 640x480 */
			line([1.0; 4], 1.0, [0.0, 0.0, 640.0, 0.0], c.transform, g);
			line([1.0; 4], 1.0, [640.0, 0.0, 640.0, 480.0], c.transform, g);
			line([1.0; 4], 1.0, [640.0, 480.0, 0.0, 480.0], c.transform, g);
			line([1.0; 4], 1.0, [0.0, 0.0, 0.0, 480.0], c.transform, g);
			/* window end */
			
			/* game field */
			let x1 = 10.0;
			let y1 = 10.0;
			line([1.0; 4], 1.0, [x1, y1, x1 + 202.0, y1], c.transform, g);
			line([1.0; 4], 1.0, [x1 + 202.0, y1, x1 + 202.0, y1 + 402.0], c.transform, g);
			line([1.0; 4], 1.0, [x1 + 202.0, y1 + 402.0, x1, y1 + 402.0], c.transform, g);
			line([1.0; 4], 1.0, [x1, y1, x1, y1 + 402.0], c.transform, g);
			
			let square_side = 20_f64; //px
			for i in 0..self.level.width() {
				for j in 0..self.level.height() {
					let x2 = i as f64 * square_side + x1 + 1.0;
					let y2 = j as f64 * square_side + y1 + 1.0;
					if self.level.get(i, j) {
						rectangle([0.86, 0.86, 0.86, 1.0], [x2, y2, square_side, square_side], c.transform, g);
					}
					if self.level.get_figure(i, j) {
						rectangle([1.0, 0.0, 0.0, 1.0], [x2, y2, square_side, square_side], c.transform, g);
					}
				}
			}
			/* game field end */			
        });
	}
	
	pub fn update(&mut self, args: UpdateArgs) {
		self.prev_time += args.dt;
		if self.prev_time > 1.0 {
			self.level.step();
			self.prev_time = 0.0;
		}
    }

}
