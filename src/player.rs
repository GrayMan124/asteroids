use macroquad::prelude::*;
mod position;

const PLAYER_SPEED: f32 = 10.;

pub struct Player {
    pub pos: Vec2,
    curr_force: Vec2,
    speed: f32,
}

impl Player {
    pub fn new(position: Vec2) -> Player {
        Player {
            pos: position,
            curr_force: Vec2::new(0.0, -5.0),
            speed: PLAYER_SPEED,
        }
    }
    pub fn draw_player(&self) {
        draw_triangle(
            Vec2 {
                x: self.pos.x,
                y: self.pos.y - 30.,
            },
            Vec2 {
                x: self.pos.x - 10.,
                y: self.pos.y - 5.,
            },
            Vec2 {
                x: self.pos.x + 10.,
                y: self.pos.y - 5.,
            },
            WHITE,
        );
    }
    pub fn update_pos(&mut self, delta: f32) {
        self.pos += self.curr_force * delta * self.speed;
    }
}
