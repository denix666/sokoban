use macroquad::prelude::*;

mod res;
use res::Resources;

mod player;
use player::Player;

mod package;
use package::Package;

mod points;
use points::*;

mod map;
use map::*;

mod game;
use game::{Game, draw_score};

pub enum GameState {
    Game,
    Intro,
    InitLevel,
    LevelCompleted,
    GameCompleted,
}

fn window_conf() -> Conf {
    let mut title = String::from("Sokoban v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: 610,
        window_height: 550,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new().await;
    let mut points: Vec<Point> = make_map_array(1);
    let resources = Resources::new().await;
    let mut player = Player::new().await;
    let mut packages: Vec<Package> = Vec::new();


    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                draw_texture(resources.bg_texture, 0.0,0.0, WHITE);
                draw_texture(resources.intro_texture, 0.0,0.0, WHITE);
                
                if is_key_pressed(KeyCode::Space) {
                    game.moves = 0;
                    game.lvl_num = 1;
                    game_state = GameState::InitLevel;
                }
            },
            GameState::Game => {
                draw_map(&points, &resources);

                if is_key_pressed(KeyCode::Left) {
                    let check_x: i32 = ((player.x - 50.0) / 50.0) as i32;
                    let check_y: i32 = (player.y / 50.0) as i32;
                    
                    let mut can_move_package: bool = true;

                    if crate::map::get_val(check_x, check_y, &points) != "#" {
                        for package in &mut packages {
                            if check_x - 1 == (package.x / 50.0) as i32 && check_y == (package.y / 50.0) as i32 {
                                can_move_package = false;
                            }
                        }
                        for package in &mut packages {
                            if check_x == (package.x / 50.0) as i32 && check_y == (package.y / 50.0) as i32 {
                                let check_px: i32 = ((package.x - 50.0) / 50.0) as i32;
                                let check_py: i32 = (package.y / 50.0) as i32;
                    
                                if crate::map::get_val(check_px, check_py, &points) != "#" {
                                    if can_move_package {
                                        package.x -= 50.0;
                                    } else {
                                        player.x += 50.0;
                                    }
                                } else {
                                    player.x += 50.0;
                                }
                            }
                        }
                        player.x -= 50.0;
                        game.moves += 1;
                    }
                }

                if is_key_pressed(KeyCode::Right) {
                    let check_x: i32 = ((player.x + 50.0) / 50.0) as i32;
                    let check_y: i32 = (player.y / 50.0) as i32;

                    let mut can_move_package: bool = true;
                    
                    if crate::map::get_val(check_x, check_y, &points) != "#" {
                        for package in &mut packages {
                            if check_x + 1 == (package.x / 50.0) as i32 && check_y == (package.y / 50.0) as i32 {
                                can_move_package = false;
                            }
                        }
                        for package in &mut packages {
                            if check_x == (package.x / 50.0) as i32 && check_y == (package.y / 50.0) as i32 {
                                let check_px: i32 = ((package.x + 50.0) / 50.0) as i32;
                                let check_py: i32 = (package.y / 50.0) as i32;
                    
                                if crate::map::get_val(check_px, check_py, &points) != "#" {
                                    if can_move_package {
                                        package.x += 50.0;
                                    } else {
                                        player.x -= 50.0;
                                    }
                                } else {
                                    player.x -= 50.0;
                                }
                            }
                        }
                        player.x += 50.0;
                        game.moves += 1;
                    }
                }

                if is_key_pressed(KeyCode::Up) {
                    let check_x: i32 = (player.x / 50.0) as i32;
                    let check_y: i32 = ((player.y - 50.0) / 50.0) as i32;

                    let mut can_move_package: bool = true;
                    
                    if crate::map::get_val(check_x, check_y, &points) != "#" {
                        for package in &mut packages {
                            if check_x == (package.x / 50.0) as i32 && check_y - 1 == (package.y / 50.0) as i32 {
                                can_move_package = false;
                            }
                        }
                        for package in &mut packages {
                            if check_x == (package.x / 50.0) as i32 && check_y == (package.y / 50.0) as i32 {
                                let check_px: i32 = (package.x / 50.0) as i32;
                                let check_py: i32 = ((package.y - 50.0) / 50.0) as i32;
                    
                                if crate::map::get_val(check_px, check_py, &points) != "#" {
                                    if can_move_package {
                                        package.y -= 50.0;
                                    } else {
                                        player.y += 50.0;
                                    }
                                } else {
                                    player.y += 50.0;
                                }
                            }
                        }
                        player.y -= 50.0;
                        game.moves += 1;
                    }
                }

                if is_key_pressed(KeyCode::Down) {
                    let check_x: i32 = (player.x / 50.0) as i32;
                    let check_y: i32 = ((player.y + 50.0) / 50.0) as i32;

                    let mut can_move_package: bool = true;
                    
                    if crate::map::get_val(check_x, check_y, &points) != "#" {
                        for package in &mut packages {
                            if check_x == (package.x / 50.0) as i32 && check_y + 1 == (package.y / 50.0) as i32 {
                                can_move_package = false;
                            }
                        }
                        for package in &mut packages {
                            if check_x == (package.x / 50.0) as i32 && check_y == (package.y / 50.0) as i32 {
                                let check_px: i32 = (package.x / 50.0) as i32;
                                let check_py: i32 = ((package.y + 50.0) / 50.0) as i32;
                    
                                if crate::map::get_val(check_px, check_py, &points) != "#" {
                                    if can_move_package {
                                        package.y += 50.0;
                                    } else {
                                        player.y -= 50.0;
                                    }
                                } else {
                                    player.y -= 50.0;
                                }
                            }
                        }
                        player.y += 50.0;
                        game.moves += 1;
                    }
                }

                if is_key_pressed(KeyCode::R) {
                    game.moves = 0;
                    game_state = GameState::InitLevel;
                }

                if is_key_pressed(KeyCode::N) {
                    if game.lvl_num == 20 {
                        game.lvl_num = 1;
                    } else {
                        game.lvl_num += 1;
                    }
                    game.moves = 0;
                    game_state = GameState::InitLevel;
                }

                for package in &mut packages {
                    package.draw(&resources);
                }

                if game::all_packages_in_place(&packages, &points) {
                    if game.lvl_num == 20 {
                        game.lvl_num = 1;
                    } else {
                        game.lvl_num += 1;
                    }
                    game.moves = 0;
                    game_state = GameState::InitLevel;
                }

                player.draw(&resources);

                draw_score(&resources, &game);
            },
            GameState::InitLevel => {
                packages.clear();
                points.clear();
                points = make_map_array(game.lvl_num);
                
                // load player position
                for point in &mut points {
                    match point.value.as_str() {
                        "@" => {
                            player.x = point.x as f32 * 50.0;
                            player.y = point.y as f32 * 50.0;
                            break;
                        },
                        _ => {
                        },
                    }
                }

                // load packages positions (can have more than one)
                for point in &mut points {
                    match point.value.as_str() {
                        "$" => {
                            packages.push(
                                Package::new(point.x as f32 * 50.0, point.y as f32 * 50.0).await
                            );
                        },
                        "*" => {
                            packages.push(
                                Package::new(point.x as f32 * 50.0, point.y as f32 * 50.0).await
                            );
                        },
                        _ => {},
                    }
                }

                game_state = GameState::Game;
            },
            GameState::LevelCompleted => {},
            GameState::GameCompleted => {}, 
        }

        next_frame().await
    }
}