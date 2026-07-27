use macroquad::prelude::*;

const PROJECTILE_SPEED: f32 = 20.;
const PROJECTILE_RADIUS: f32 = 2.;

#[derive(Copy, Clone)]
pub struct Projectile {
    pub pos: Vec2,
    curr_force: Vec2,
    speed: f32,
    pub alive: bool,
}

impl Projectile {
    pub fn new(position: Vec2, force: Vec2) -> Projectile {
        Projectile {
            pos: position,
            curr_force: force,
            speed: PROJECTILE_SPEED,
            alive: true,
        }
    }

    pub fn update_pos(&mut self, delta: f32) {
        self.pos += self.curr_force * delta * self.speed;
    }

    pub fn draw(&self) {
        draw_circle(self.pos[0], self.pos[1], PROJECTILE_RADIUS, WHITE);
    }
}
