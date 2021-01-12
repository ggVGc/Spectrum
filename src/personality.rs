use crate::speck::Speck;
use macroquad::prelude::*;

pub struct Personality {
  pub racism: Racism,
  pub loner: f32,
  pub stamina: f32,
}

pub struct Racism {
  pub weight: f32,
  pub target: usize,
}

pub fn dir_from_personality(pos: Vec2, personality: &Personality, neighbours: Vec<&Speck>) -> Vec2 {
  (neighbours[0].pos - pos).normalize()
}
