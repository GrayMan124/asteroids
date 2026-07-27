use macroquad::prelude::*;

const ASTEROID_SPEED: f32 = 10.;
const ASTEROID_RADIUS: f32 = 7.;
const ASTEROID_THICKNESS: f32 = 2.;

pub struct Asteroid {
    pub pos: Vec2,
    curr_force: Vec2,
    size: f32,
    speed: f32,
}

impl Asteroid {
    pub fn new(position: Vec2, force: Vec2) -> Asteroid {
        Asteroid {
            pos: position,
            curr_force: force,
            speed: ASTEROID_SPEED,
            size: ASTEROID_RADIUS,
        }
    }

    pub fn update_pos(&mut self, delta: f32) {
        self.pos += self.curr_force * delta * self.speed;
    }

    pub fn draw(&self) {
        draw_circle_lines(
            self.pos[0],
            self.pos[1],
            ASTEROID_RADIUS,
            ASTEROID_THICKNESS,
            WHITE,
        );
    }
}
