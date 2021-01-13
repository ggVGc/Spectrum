use crate::rand::gen_range;
use crate::speck::Speck;
use macroquad::prelude::*;

pub struct Personality {
  pub stamina: f32,

  pub racism: Racism,
  pub loner: Loner,
  pub stalker: Stalker,
}

pub struct Racism {
  pub weight: f32,
  pub target: usize,
}

pub struct Stalker {
  pub target: Option<i32>,
  pub weight: f32,
}

pub struct Loner {
  pub direction: f32,
  pub weight: f32,
}

pub fn rand_personality(my_color: usize, color_count: usize) -> Personality {
  Personality {
    stamina: min_rand_weight(0.4),
    racism: Racism {
      weight: gen_range(0.0, 1.0),
      target: rand_racism_target(my_color, color_count),
    },
    loner: Loner {
      direction: if gen_range(0.0, 1.0) > 0.5 { 1.0 } else { -1.0 },
      weight: gen_range(0.0, 1.0),
    },
    stalker: Stalker {
      weight: gen_range(0.0, 1.0),
      target: None,
    },
  }
}
fn rand_racism_target(my_color: usize, color_count: usize) -> usize {
  let colors: Vec<_> = (0..color_count).collect();
  let colors: Vec<_> = colors.iter().filter(|c| **c != my_color).collect();
  let ind = gen_range(0, colors.len());
  *colors[ind]
}

fn min_rand_weight(min: f32) -> f32 {
  gen_range(min, 1.0)
}

pub fn dir_from_personality(pos: Vec2, personality: &Personality, neighbours: &[&Speck]) -> Vec2 {
  let dir = racism_dir(pos, &personality.racism, neighbours)
    + loner_dir(pos, &personality.loner, neighbours);

  safe_normalize(dir)
}

fn safe_normalize(vec: Vec2) -> Vec2 {
  if vec.length() > 0.0 {
    vec.normalize()
  } else {
    vec
  }
}

fn racism_dir(pos: Vec2, racism: &Racism, neighbours: &[&Speck]) -> Vec2 {
  let mut dir = vec2(0.0, 0.0);
  for speck in neighbours {
    if speck.color_index == racism.target {
      dir -= speck.pos - pos;
    }
  }

  safe_normalize(dir * racism.weight)
}

fn loner_dir(pos: Vec2, loner: &Loner, neighbours: &[&Speck]) -> Vec2 {
  let mut dir = vec2(0.0, 0.0);
  for speck in neighbours {
    dir += (speck.pos - pos) * loner.direction;
  }

  safe_normalize(dir * loner.weight)
}
