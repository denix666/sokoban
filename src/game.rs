use macroquad::prelude::*;
use crate::{res::Resources, package::Package, points::Point};

pub fn show_message(res: &Resources, msg: &str) {
    draw_text_ex(msg, 15.0, 320.0, 
        TextParams {
            font: res.font,
            font_size: 27,
            color: GOLD,
            ..Default::default()
        },
    );
}

pub fn draw_score(res: &Resources, game: &Game) {
    let mut moves_text = String::from("Moves: ");
    moves_text.push_str(&game.moves.to_string());
    draw_text_ex(&moves_text, 15.0, 620.0, 
        TextParams {
            font: res.font,
            font_size: 27,
            color: GOLD,
            ..Default::default()
        },
    );

    let mut level_text = String::from("Level: ");
    level_text.push_str(&game.lvl_num.to_string());
    draw_text_ex(&level_text, 15.0, 650.0, 
        TextParams {
            font: res.font,
            font_size: 27,
            color: GOLD,
            ..Default::default()
        },
    );

    draw_text_ex("R - Restart level", 200.0, 620.0, 
        TextParams {
            font: res.font,
            font_size: 27,
            color: GOLD,
            ..Default::default()
        },
    );

    draw_text_ex("N - Jump to next level", 200.0, 650.0, 
        TextParams {
            font: res.font,
            font_size: 27,
            color: GOLD,
            ..Default::default()
        },
    );
}

pub fn all_packages_in_place(packages: &Vec<Package>, points: &Vec<Point>) -> bool {
    let mut result: bool = true;

    for package in packages {
        let check_px: i32 = (package.x / 64.0) as i32;
        let check_py: i32 = (package.y / 64.0) as i32;
        if crate::map::get_val(check_px, check_py, &points) != "." && crate::map::get_val(check_px, check_py, &points) != "*" {
            result = false;
            break;
        }
    }

    return result;
}

pub struct Game {
    pub moves: i32,
    pub lvl_num: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            moves: 0,
            lvl_num: 0,
        }
    }
}
