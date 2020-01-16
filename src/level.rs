use rand::prelude::*;
use std::fmt;

use crate::figure::*;

const ZONE_WIDTH: usize = 10;
const ZONE_HEIGHT: usize = 20;

pub type Zone = Vec<Vec<bool>>;

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
	
	pub fn step(&mut self) {
		match &mut self.figure {
			Some(f) => {
				let y = f.y() + 1;
				if (y + f.height() - 1) < ZONE_HEIGHT {
					let mut is_cross = false;
					for i in 0..f.width() {
						for j in 0..f.height() {
							if f.zone()[i][j] && self.zone[f.x() + i][y + j] {
								is_cross = true;
							}
						}
					}
					if !is_cross {
						f.set_y(y);	
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
			},
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
				self.figure = Some(create_figure(rng.gen_range(0, 7)));
			}
		}
	}
	
	pub fn move_right(&mut self) {
		match &mut self.figure {
			Some(f) => {
				if f.x() + f.width() < 10 {
					f.move_right();
				}
			},
			_ => println!("no figure"),
		}
	}
	
	pub fn move_left(&mut self) {
		match &mut self.figure {
			Some(f) => {
				if f.x() > 0 {
					f.move_left();
				}	
			},
			_ => println!("no figure"),
		}
	}
	
	pub fn rotate(&mut self) {
		match &mut self.figure {
			Some(f) => f.rotate(),
			_ => println!("no figure"),
		}
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
