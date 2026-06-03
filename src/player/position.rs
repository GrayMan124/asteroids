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
}
