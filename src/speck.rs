use crate::personality::{Personality, Racism};
use macroquad::prelude::*;
use crate::rand::gen_range;

const MAX_AGE : i32 = 100;

pub struct Speck {
  pub id: i32,
  pub pos: Vec2,
  pub color_index: usize,
  pub dir: Vec2,
  pub personality: Personality,
  pub age: i32
}

pub fn rand_speck(id: i32, color_count: usize, half_canvas_size: f32) -> Speck {
  Speck {
    id: id,
    color_index: gen_range(0, color_count),
    pos: vec2(
      gen_range(-half_canvas_size, half_canvas_size),
      gen_range(-half_canvas_size, half_canvas_size),
    ),
    dir: vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)).normalize(),
    personality: rand_personality(color_count),
    age: gen_range(0, MAX_AGE / 2)
  }
}

fn rand_personality(color_count: usize) -> Personality {
  Personality {
    racism: Racism {
      weight: rand_weight(),
      target: gen_range(0, color_count),
    },
    loner: rand_weight(),
    stamina: rand_weight(),
  }
}

fn rand_weight() -> f32 {
  gen_range(0.0, 1.0)
}
