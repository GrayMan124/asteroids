use macroquad::prelude::*;
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
    let mut player_v1 = Vec2::new(240., 240.);
    let mut player_v2 = Vec2::new(280., 240.);
    let mut player_v3 = Vec2::new(260., 200.);
    let mut player = player::Player::new(player_v1, player_v2, player_v3);
    loop {
        clear_background(BLACK);
        player.draw_player();
        let delta = get_frame_time();
        player.update_pos(delta);
        next_frame().await
    }
}
