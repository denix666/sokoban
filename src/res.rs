use macroquad::prelude::*;

pub struct Resources {
    pub wall_texture: Texture2D,
    pub box_texture: Texture2D,
    pub player_texture: Texture2D,
    pub point_texture: Texture2D,
    pub intro_texture: Texture2D,
    pub floor_texture: Texture2D,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            wall_texture: load_texture("assets/images/wall.png").await.unwrap(),
            box_texture: load_texture("assets/images/box.png").await.unwrap(),
            player_texture: load_texture("assets/images/player.png").await.unwrap(),
            point_texture: load_texture("assets/images/point.png").await.unwrap(),
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            floor_texture: load_texture("assets/images/floor.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
        }
    }
}