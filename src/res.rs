use macroquad::prelude::*;

pub struct Resources {
    pub wall_texture: Texture2D,
    pub box_texture: Texture2D,
    pub player_texture: Texture2D,
    pub point_texture: Texture2D,
    pub bg_texture: Texture2D,
    pub intro_texture: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            wall_texture: load_texture("assets/images/wall.png").await.unwrap(),
            box_texture: load_texture("assets/images/box.png").await.unwrap(),
            player_texture: load_texture("assets/images/player.png").await.unwrap(),
            point_texture: load_texture("assets/images/point.png").await.unwrap(),
            bg_texture: load_texture("assets/images/bg.png").await.unwrap(),
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
        }
    }
}