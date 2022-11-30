use macroquad::prelude::*;
use crate::{res::Resources};

pub struct Player {
    pub x: f32,
    pub y: f32,
}


impl Player {
    pub async fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn draw(&mut self, res: &Resources) {
        draw_texture(res.player_texture, self.x ,self.y, WHITE);
    }
}
