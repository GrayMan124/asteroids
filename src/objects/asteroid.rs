use macroquad::prelude::*;

const ASTEROID_THICKNESS: f32 = 1.5;

pub struct Asteroid {
    pub pos: Vec2,
    curr_force: Vec2,
    pub size: f32,
    speed: f32,
    pub alive: bool,
}

impl Asteroid {
    pub fn new(position: Vec2, force: Vec2) -> Asteroid {
        Asteroid {
            pos: position,
            curr_force: force,
            speed: rand::gen_range(10., 50.),
            size: rand::gen_range(3.0, 25.),
            alive: true,
        }
    }

    pub fn update_pos(&mut self, delta: f32) {
        self.pos += self.curr_force * delta * self.speed;
    }

    pub fn draw(&self) {
        draw_circle_lines(
            self.pos[0],
            self.pos[1],
            self.size,
            ASTEROID_THICKNESS,
            WHITE,
        );
    }
}
