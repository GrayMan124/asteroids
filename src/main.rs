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
    let mut player = player::Player::new(Vec2::new(240., 240.));
    loop {
        clear_background(BLACK);
        player.draw_player();
        let delta = get_frame_time();
        player.update_pos(delta);
        // if is_key_down(KeyCode::Space) {
        //     player.rotate(0.0);
        // }
        // println!(
        //     "Current position of tip {} {}",
        //     player.pos.v1[0], player.pos.v1[1]
        // );
        next_frame().await
    }
}
