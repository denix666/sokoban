use macroquad::prelude::*;
use crate::res::Resources;

pub struct Package {
    pub x: f32,
    pub y: f32,
}

impl Package {
    pub async fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn draw(&mut self, res: &Resources) {
        draw_texture(res.box_texture, self.x ,self.y, WHITE);
    }
}
