use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub moves: i32,
    pub lvl_num: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            moves: 0,
            lvl_num: 0,
        }
    }
}
