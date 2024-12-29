use bevy::prelude::*;
use rand::Rng;

pub mod bmath;

pub fn random_direction() -> Vec2 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-1.0..1.0);
    let y = rng.gen_range(-1.0..1.0);

    Vec2::new(x, y).normalize()
}
