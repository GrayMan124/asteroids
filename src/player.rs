use macroquad::prelude::*;
mod position;

const PLAYER_SPEED: f32 = 10.;

pub struct Player {
    pos: position::Position,
    curr_force: Vec2,
    speed: f32,
}

impl Player {
    pub fn new(v1: Vec2, v2: Vec2, v3: Vec2) -> Player {
        Player {
            pos: position::Position::new(v1, v2, v3),
            curr_force: Vec2::new(0.0, -10.0),
            speed: PLAYER_SPEED,
        }
    }
    pub fn draw_player(&self) {
        draw_triangle(self.pos.v1, self.pos.v2, self.pos.v3, WHITE);
    }
    pub fn update_pos(&mut self, delta: f32) {
        self.pos.v1 += self.curr_force * delta * self.speed;
        self.pos.v2 += self.curr_force * delta * self.speed;
        self.pos.v3 += self.curr_force * delta * self.speed;
    }
}
