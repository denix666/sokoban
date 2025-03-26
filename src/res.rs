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
            wall_texture: Texture2D::from_file_with_format(include_bytes!("../assets/images/wall.png"), None),
            box_texture: Texture2D::from_file_with_format(include_bytes!("../assets/images/box.png"), None),
            player_texture: Texture2D::from_file_with_format(include_bytes!("../assets/images/player.png"), None),
            point_texture: Texture2D::from_file_with_format(include_bytes!("../assets/images/point.png"), None),
            intro_texture: Texture2D::from_file_with_format(include_bytes!("../assets/images/intro.png"), None),
            floor_texture: Texture2D::from_file_with_format(include_bytes!("../assets/images/floor.png"), None),
            font: load_ttf_font_from_bytes(include_bytes!("../assets/fonts/game_font.ttf")).unwrap(),
        }
    }
}
