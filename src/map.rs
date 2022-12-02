use macroquad::prelude::*;
use crate::points::Point;
use crate::res::Resources;

pub fn get_val(check_x: i32, check_y: i32, points: &Vec<Point>) -> String {
    let result = match points.iter().position(|x| x.x == check_x && x.y == check_y) {
        Some(idx) => points[idx].value.to_string(),
        _ => String::from("empty"),
    };
    return result
}

pub fn draw_map(points: &Vec<Point>, res: &Resources) {
    for point in points {
        match point.value.as_str() {
            "#" => {
                draw_texture(res.floor_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
                draw_texture(res.wall_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
            },
            "." => {
                draw_texture(res.floor_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
                draw_texture(res.point_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
            },
            "*" => {
                draw_texture(res.floor_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
                draw_texture(res.point_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
            },
            _ => {
                draw_texture(res.floor_texture, point.x as f32 * 64.0,point.y as f32 * 64.0, WHITE);
            },
        };
    }
}

pub fn make_map_array(lvl_num: i32) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let map = match lvl_num {
        1 => vec![
            " #######    ",
            " #     #    ",
            " # .$. #    ",
            "## $@$ #    ",
            "#  .$. #    ",
            "#      #    ",
            "########    ",
            "            ",
            "            "],
        2 => vec![
            "######      ",
            "#    #      ",
            "# #@ #      ",
            "# $* #      ",
            "# .* #      ",
            "#    #      ",
            "######      ",
            "            ",
            "            "],
        3 => vec![
            "  ####      ",
            "###  ####   ",
            "#     $ #   ",
            "# #  #$ #   ",
            "# . .#@ #   ",
            "#########   ",
            "            ",
            "            ",
            "            "],
        4 => vec![   
            "####        ",
            "# .#        ",
            "#  ###      ",
            "#*@  #      ",
            "#  $ #      ",
            "#  ###      ",
            "####        ",
            "            ",
            "            "],
        5 => vec![   
            "            ",
            "  ########  ",
            "  #      #  ",
            "  # .**$@#  ",
            "  #      #  ",
            "  #####  #  ",
            "      ####  ",
            "            ",
            "            "],
        6 => vec![   
            "###### #####",
            "#    ###   #",
            "# $$     #@#",
            "# $ #...   #",
            "#   ########",
            "#####       ",
            "            ",
            "            ",
            "            "],
        7 => vec![   
            "#######     ",
            "#     #     ",
            "# .$. #     ",
            "# $.$ #     ",
            "# .$. #     ",
            "# $.$ #     ",
            "#  @  #     ",
            "#######     ",
            "            "],
        8 => vec![   
            "#####       ",
            "#.  ##      ",
            "#@$$ #      ",
            "##   #      ",
            " ##  #      ",
            "  ##.#      ",
            "   ###      ",
            "            ",
            "            "],
        9 => vec![    
            "      ##### ",
            "      #.  # ",
            "      #.# # ",
            "#######.# # ",
            "# @ $ $ $ # ",
            "# # # # ### ",
            "#       #   ",
            "#########   ",
            "            "],
        10 => vec![
            "  ######    ",
            "  #    #    ",
            "  # ##@##   ",
            "### # $ #   ",
            "# ..# $ #   ",
            "#       #   ",
            "#  ######   ",
            "####        ",
            "            "],
        11 => vec![
            "#####       ",
            "#   ##      ",
            "# $  #      ",
            "## $ ####   ",
            " ###@.  #   ",
            "  #  .# #   ",
            "  #     #   ",
            "  #######   ",
            "            "],
        12 => vec![
            "####        ",
            "#. ##       ",
            "#.@ #       ",
            "#. $#       ",
            "##$ ###     ",
            " # $  #     ",
            " #    #     ",
            " #  ###     ",
            " ####       "],
        13 => vec![
            "#######     ",
            "#     #     ",
            "# # # #     ",
            "#. $*@#     ",
            "#   ###     ",
            "#####       ",
            "            ",
            "            ",
            "            "],
        14 => vec![
            "     ###    ",
            "######@##   ",
            "#    .* #   ",
            "#   #   #   ",
            "#####$# #   ",
            "    #   #   ",
            "    #####   ",
            "            ",
            "            "],
        15 => vec![
            " ####       ",
            " #  ####    ",
            " #     ##   ",
            "## ##   #   ",
            "#. .# @$##  ",
            "#   # $$ #  ",
            "#  .#    #  ",
            "##########  ",
            "            "],
        16 => vec![
            "#####       ",
            "# @ #       ",
            "#...#       ",
            "#$$$##      ",
            "#    #      ",
            "#    #      ",
            "######      ",
            "            ",
            "            "],
        17 => vec![
            "#######     ",
            "#     #     ",
            "#. .  #     ",
            "# ## ##     ",
            "#  $ #      ",
            "###$ #      ",
            "  #@ #      ",
            "  #  #      ",
            "  ####      "],
        18 => vec![
            "########    ",
            "#   .. #    ",
            "#  @$$ #    ",
            "##### ##    ",
            "   #  #     ",
            "   #  #     ",
            "   #  #     ",
            "   ####     ",
            "            "],
        19 => vec![
            "#######     ",
            "#     ###   ",
            "#  @$$..#   ",
            "#### ## #   ",
            "  #     #   ",
            "  #  ####   ",
            "  #  #      ",
            "  ####      ",
            "            "],
        20 => vec![
            "####        ",
            "#  ####     ",
            "# . . #     ",
            "# $$#@#     ",
            "##    #     ",
            " ######     ",
            "            ",
            "            ",
            "            "],
        _ => panic!("no such level"),
    };
    
    let mut mx: i32 = 0;
    let mut my: i32 = 0;
    for line in map {
        for c in line.chars() {
            points.push(
                Point::new(mx,my,c.to_string()),
            );
            mx += 1;
        }
        my += 1;
        mx = 0;
    }

    return points
}