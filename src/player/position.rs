use macroquad::prelude::*;

pub struct Position {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
}

impl Position {
    pub fn new(v1: Vec2, v2: Vec2, v3: Vec2) -> Position {
        Position { v1, v2, v3 }
    }
    pub fn rotate(&mut self, angle: f32) {
        self.v1 = self.v1.rotate(vec2(1., 1.));
        // self.v2 = self.v2.rotate(vec2(30., 30.));
        // self.v3 = self.v3.rotate(vec2(30., 30.));
    }
}
