// use macroquad::math;
use macroquad::prelude::*;
mod asteroid;
pub mod projectile;

const ASTEROIDS_SPAWN_RATE: f64 = 2.;

pub struct Projectiles {
    list: Vec<projectile::Projectile>,
}

impl Projectiles {
    pub fn new() -> Projectiles {
        Projectiles { list: Vec::new() }
    }
    pub fn add_proj(&mut self, proj: projectile::Projectile) {
        self.list.push(proj);
    }
    pub fn update_pos(&mut self, delta: f32) {
        for proj in self.list.iter_mut() {
            proj.update_pos(delta);
        }
    }
    pub fn draw(&mut self) {
        for proj in self.list.iter() {
            proj.draw();
        }
    }
    pub fn clean_up(&mut self, screen_width: f32, screen_height: f32) {
        self.list.retain(|proj| {
            proj.pos[0] > 0.
                && proj.pos[0] < screen_width
                && proj.pos[1] > 0.
                && proj.pos[1] < screen_height
                && proj.alive
        });
    }
}

pub struct Asteroids {
    pub list: Vec<asteroid::Asteroid>,
    last_spawn: f64,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        Asteroids {
            list: Vec::new(),
            last_spawn: 0.0,
        }
    }
    pub fn spawn(&mut self, time: f64, scr_w: f32, scr_h: f32) {
        if time - self.last_spawn > ASTEROIDS_SPAWN_RATE {
            let start_pos = rand::gen_range(0, 4);
            let created_asteroid = match start_pos {
                0 => asteroid::Asteroid::new(
                    Vec2::new(rand::gen_range(0.0, scr_w), 0.0),
                    Vec2::new(0.0, 1.0),
                ),
                1 => asteroid::Asteroid::new(
                    Vec2::new(0.0, rand::gen_range(0.0, scr_h)),
                    Vec2::new(1.0, 0.0),
                ),
                2 => asteroid::Asteroid::new(
                    Vec2::new(rand::gen_range(0.0, scr_w), scr_h),
                    Vec2::new(0.0, -1.0),
                ),
                3 => asteroid::Asteroid::new(
                    Vec2::new(scr_w, rand::gen_range(0.0, scr_h)),
                    Vec2::new(-1.0, 0.0),
                ),
                _ => {
                    println!("Incorrect start position for creating an asteroid");
                    asteroid::Asteroid::new(
                        Vec2::new(scr_w, rand::gen_range(0.0, scr_h)),
                        Vec2::new(1.0, 0.0),
                    )
                }
            };
            self.list.push(created_asteroid);
            self.last_spawn = time;
        }
    }
    pub fn update_pos(&mut self, delta: f32) {
        for astr in self.list.iter_mut() {
            astr.update_pos(delta);
        }
    }
    pub fn draw(&mut self, texture_1: &Texture2D, texture_2: &Texture2D) {
        for astr in self.list.iter() {
            let mut texture = texture_1;
            if rand::gen_range(0, 2) == 1 {
                texture = texture_1;
            }
            astr.draw(texture);
        }
    }
    pub fn clean_up(&mut self, screen_width: f32, screen_height: f32) {
        self.list.retain(|astr| {
            astr.pos[0] > 0.
                && astr.pos[0] < screen_width
                && astr.pos[1] > 0.
                && astr.pos[1] < screen_height
                && astr.alive
        });
    }
}

pub fn check_collsion_asteroids(
    asteroids_list: &mut Asteroids,
    projectiles_list: &mut Projectiles,
) -> i32 {
    let mut score = 0;
    for astr in asteroids_list.list.iter_mut() {
        for proj in projectiles_list.list.iter_mut() {
            if astr.pos.distance(proj.pos) < astr.size {
                astr.alive = false;
                proj.alive = false;
                score += 1;
            }
        }
    }
    score
}
