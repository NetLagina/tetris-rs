pub struct Settings {
    pub game_width: u32,
    pub game_height: u32,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            game_width: 640,
            game_height: 480,
        }
    }
}
