use macroquad::prelude::*;
// use macroquad::rand;
use macroquad::time;

// use crate::GameState::MainMenu;
use crate::objects::check_collsion_asteroids;
mod objects;
mod player;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[macroquad::main("Asteroids")]
async fn main() {
    const SCR_H: f32 = 480.0;
    const SCR_W: f32 = 480.0;
    set_camera(&Camera2D {
        zoom: vec2(1. / SCR_W * 2., 1. / SCR_H * 2.),
        target: vec2(SCR_W / 2., SCR_H / 2.),
        ..Default::default()
    });
    let mut projectiles_list: objects::Projectiles = objects::Projectiles::new();
    let mut asteroids_list: objects::Asteroids = objects::Asteroids::new();
    let mut player = player::Player::new(Vec2::new(240., 240.));
    let mut last_fps_check = 0.0;
    let mut score = 0;
    let mut game_state = GameState::MainMenu;
    loop {
        match game_state {
            GameState::MainMenu => {
                clear_background(BLACK);
                let (font_size, font_scale, font_aspect) = camera_font_scale(80.);
                let title_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(&format!("Asteroids"), SCR_W / 3.3, SCR_H / 2., title_params);
                let (font_size, font_scale, font_aspect) = camera_font_scale(30.);
                let space_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(
                    &format!("Press Space"),
                    SCR_W / 2.,
                    SCR_H / 2. + 40.,
                    space_params,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                clear_background(BLACK);
                let (font_size, font_scale, font_aspect) = camera_font_scale(20.);
                let text_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(
                    &format!("Score: {scr}", scr = score),
                    SCR_W / 2.,
                    10.,
                    text_params,
                );
                let delta = get_frame_time();
                player.update_pos(delta);
                if is_key_down(KeyCode::Left) {
                    player.rotate(-1.);
                }
                if is_key_down(KeyCode::Right) {
                    player.rotate(1.);
                }
                if is_key_down(KeyCode::Space) {
                    player.thrust();
                }
                if is_key_pressed(KeyCode::E) {
                    match player.shoot(time::get_time()) {
                        Some(projectile) => {
                            projectiles_list.add_proj(projectile);
                        }
                        None => {}
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                projectiles_list.draw();
                projectiles_list.update_pos(delta);
                asteroids_list.spawn(time::get_time(), SCR_W, SCR_H);
                asteroids_list.draw();
                asteroids_list.update_pos(delta);
                score += check_collsion_asteroids(&mut asteroids_list, &mut projectiles_list);
                asteroids_list.clean_up(SCR_W, SCR_H);
                projectiles_list.clean_up(SCR_W, SCR_H);
                player.draw_player();
                if player.check_collision(&asteroids_list) {
                    game_state = GameState::GameOver;
                };
                if time::get_time() - last_fps_check > 1. {
                    println!("Current fps: {}", time::get_fps());
                    last_fps_check = time::get_time();
                }
                next_frame().await
            }
            GameState::Paused => {
                clear_background(BLACK);
                let (font_size, font_scale, font_aspect) = camera_font_scale(80.);
                let title_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(
                    &format!("Game Paused"),
                    SCR_W / 3.3,
                    SCR_H / 2.,
                    title_params,
                );
                let (font_size, font_scale, font_aspect) = camera_font_scale(30.);
                let space_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(
                    &format!("Press Space to continue"),
                    SCR_W / 3.3,
                    SCR_H / 2. + 40.,
                    space_params,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                clear_background(BLACK);
                let (font_size, font_scale, font_aspect) = camera_font_scale(80.);
                let title_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(
                    &format!("Game Over!"),
                    SCR_W / 3.3,
                    SCR_H / 2.,
                    title_params,
                );
                let (font_size, font_scale, font_aspect) = camera_font_scale(30.);
                let space_params = TextParams {
                    font_size,
                    font_scale,
                    font_scale_aspect: font_aspect,
                    ..Default::default()
                };
                draw_text_ex(
                    &format!("Press Space to return to menu"),
                    SCR_W / 3.3,
                    SCR_H / 2. + 40.,
                    space_params,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                    projectiles_list = objects::Projectiles::new();
                    asteroids_list = objects::Asteroids::new();
                    player = player::Player::new(Vec2::new(240., 240.));
                    last_fps_check = 0.0;
                    score = 0;
                }
            }
        }
        next_frame().await
    }
}
