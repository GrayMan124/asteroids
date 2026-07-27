use std::f32::consts::PI;

use super::objects;
use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 1.;
const ATTACK_SPEED: f64 = 0.70;

pub struct Player {
    pub pos: Vec2,
    curr_force: Vec2,
    curr_dir: Vec2,
    speed: f32,
    draw_vectors: [Vec2; 3],
    offset: Vec2,
    attack_speed: f64,
    last_attack: f64,
}

impl Player {
    pub fn new(position: Vec2) -> Player {
        Player {
            pos: position,
            curr_force: Vec2::new(0.0, 0.0),
            curr_dir: Vec2::new(0.0, 1.0),
            speed: PLAYER_SPEED,
            draw_vectors: [
                Vec2::new(0., -30.),
                Vec2::new(-10., -5.),
                Vec2::new(10., -5.),
            ],
            offset: Vec2::new(0.0, 0.0),
            attack_speed: ATTACK_SPEED,
            last_attack: 0.0,
        }
    }
    pub fn draw_player(&self) {
        draw_triangle(
            Vec2 {
                x: self.pos.x + self.draw_vectors[0][0],
                y: self.pos.y + self.draw_vectors[0][1],
            },
            Vec2 {
                x: self.pos.x + self.draw_vectors[1][0],
                y: self.pos.y + self.draw_vectors[1][1],
            },
            Vec2 {
                x: self.pos.x + self.draw_vectors[2][0],
                y: self.pos.y + self.draw_vectors[2][1],
            },
            WHITE,
        );
    }
    pub fn update_pos(&mut self, delta: f32) {
        self.pos += self.curr_force * delta * self.speed;
    }
    fn calculate_avg(&self) -> Vec2 {
        let mut sum_x = 0.;
        let mut sum_y = 0.;

        for i in 0..3 {
            sum_x += self.draw_vectors[i][0];
            sum_y += self.draw_vectors[i][1];
        }

        Vec2::new(sum_x / 3., sum_y / 3.)
    }
    pub fn rotate(&mut self, dir: f32) {
        let angle = dir * PI / 48.;
        self.offset = self.calculate_avg();
        for i in 0..3 {
            self.draw_vectors[i] -= self.offset;
            self.draw_vectors[i] = Vec2::from_angle(angle).rotate(self.draw_vectors[i]);
        }
        self.curr_dir = Vec2::from_angle(angle).rotate(self.curr_dir);
    }

    pub fn thrust(&mut self) {
        self.curr_force -= self.curr_dir * PLAYER_SPEED;
    }

    pub fn shoot(&mut self, time: f64) -> Option<objects::Projectile> {
        if time - self.last_attack > self.attack_speed {
            self.last_attack = time;
            Some(objects::Projectile::new(
                Vec2::new(
                    self.pos.x + self.draw_vectors[0][0],
                    self.pos.y + self.draw_vectors[0][1],
                ),
                Vec2::new(
                    self.curr_dir.x * self.curr_force.y,
                    self.curr_dir.y * self.curr_force.y,
                ),
            ))
        } else {
            None
        }
    }
}
