use macroquad::prelude::*;
use macroquad::rand;
use macroquad::time;
mod objects;
mod player;

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
    loop {
        clear_background(BLACK);
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
        if is_key_down(KeyCode::E) {
            match player.shoot(time::get_time()) {
                Some(projectile) => {
                    projectiles_list.add_proj(projectile);
                }
                None => {}
            }
        }
        projectiles_list.draw();
        projectiles_list.update_pos(delta);
        asteroids_list.spawn(time::get_time(), SCR_W, SCR_H);
        asteroids_list.draw();
        asteroids_list.update_pos(delta);
        asteroids_list.clean_up(SCR_W, SCR_H);
        projectiles_list.clean_up(SCR_W, SCR_H);
        player.draw_player();
        next_frame().await
    }
}
