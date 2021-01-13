use crate::personality::{rand_personality, Personality};
use crate::rand::gen_range;
use macroquad::prelude::*;

pub struct Speck {
  pub id: i32,
  pub update_counter: i32,
  pub pos: Vec2,
  pub color_index: usize,
  pub dir: Vec2,
  pub personality: Personality,
  pub age: f32,
}

pub fn rand_speck(id: i32, color_count: usize, half_canvas_size: f32, max_age: f32, update_cycle: i32) -> Speck {
  let color_index = gen_range(0, color_count);
  Speck {
    id: id,
    color_index: color_index,
    update_counter: gen_range(0, update_cycle),
    pos: vec2(
      gen_range(-half_canvas_size, half_canvas_size),
      gen_range(-half_canvas_size, half_canvas_size),
    ),
    dir: vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)).normalize(),
    personality: rand_personality(color_index, color_count),
    age: gen_range(0.0, max_age),
  }
}
