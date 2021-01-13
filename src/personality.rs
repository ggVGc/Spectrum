use crate::rand::gen_range;
use crate::speck::Speck;
use macroquad::prelude::*;

pub struct Personality {
  pub stamina: f32,

  pub racism: Option<Racism>,
  pub loner: Option<Loner>,
  pub stalker: Option<Stalker>,
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
    stamina: gen_range(0.4, 1.0),
    racism: mb_make(|| Racism {
      weight: gen_range(0.0, 1.0),
      target: rand_racism_target(my_color, color_count),
    }),
    loner: mb_make(|| Loner {
      direction: fifty_fifty(1.0, -1.0),
      weight: gen_range(0.0, 1.0),
    }),
    stalker: mb_make(|| Stalker {
      weight: gen_range(0.0, 1.0),
      target: None,
    }),
  }
}

fn fifty_fifty<A>(a: A, b: A) -> A {
  if gen_range(0, 2) == 0 {
    a
  } else {
    b
  }
}

fn mb_make<A, F>(builder: F) -> Option<A>
where
  F: Fn() -> A,
{
  // if fifty_fifty(true, false) {
    Some(builder())
  // } else {
    // None
  // }
}

fn rand_racism_target(my_color: usize, color_count: usize) -> usize {
  let colors: Vec<_> = (0..color_count).collect();
  let colors: Vec<_> = colors.iter().filter(|c| **c != my_color).collect();
  let ind = gen_range(0, colors.len());
  *colors[ind]
}

pub fn dir_from_personality(pos: Vec2, personality: &Personality, neighbours: &[&Speck]) -> Vec2 {
  let dir = personality
    .racism
    .as_ref()
    .map_or(Vec2::zero(), |racism| racism_dir(pos, &racism, neighbours))
    + personality
      .loner
      .as_ref()
      .map_or(Vec2::zero(), |loner| loner_dir(pos, &loner, neighbours));

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
  let mut dir = Vec2::zero();
  for speck in neighbours {
    if speck.color_index == racism.target {
      dir -= speck.pos - pos;
    }
  }

  safe_normalize(dir * racism.weight)
}

fn loner_dir(pos: Vec2, loner: &Loner, neighbours: &[&Speck]) -> Vec2 {
  let mut dir = Vec2::zero();
  for speck in neighbours {
    dir += (speck.pos - pos) * loner.direction;
  }

  safe_normalize(dir * loner.weight)
}
