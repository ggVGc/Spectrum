use crate::rand::gen_range;
use crate::speck::Speck;
use macroquad::prelude::*;

pub struct Personality {
  pub stamina: f32,

  pub racism: Option<Racism>,
  pub loner: Option<Loner>,
  pub random_walker: Option<RandomWalker>,
  // pub stalker: Option<Stalker>,
}

pub struct Racism {
  pub weight: f32,
  pub liked_color: usize,
}

pub struct Stalker {
  pub target: Option<i32>,
  pub weight: f32,
}

pub struct RandomWalker {
  pub weight: f32,
}

pub struct Loner {
  pub weight: f32,
}

pub fn rand_personality(my_color: usize) -> Personality {
  Personality {
    stamina: gen_range(0.1, 1.0),
    racism: Some(Racism {
      liked_color: my_color,
      weight: gen_range(0.1, 1.) * 1.,
    }),
    loner: mb_make(|| Loner {
      weight: gen_range(-1., 1.) * 0.5,
    }),
    random_walker: mb_make(|| RandomWalker {
      weight: gen_range(0.01, 0.6),
    }),
    // stalker: mb_make(|| Stalker {
    //   weight: gen_range(0.0, 1.0),
    //   target: None,
    // }),
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
  if fifty_fifty(true, false) {
    Some(builder())
  } else {
    None
  }
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
  // + personality.random_walker.as_ref().map_or(Vec2::zero(), |walker| random_walker_dir(&walker));

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
  for neighbour in neighbours {
    if neighbour.color_index != racism.liked_color {
      dir -= neighbour.pos - pos;
    }
  }

  safe_normalize(dir * racism.weight)
}

fn loner_dir(pos: Vec2, loner: &Loner, neighbours: &[&Speck]) -> Vec2 {
  let mut dir = Vec2::zero();
  for speck in neighbours {
    dir += (speck.pos - pos);
  }

  safe_normalize(dir * loner.weight)
}

fn random_walker_dir(walker: &RandomWalker) -> Vec2 {
  let dir = Vec2::new(gen_range(-1., 1.), gen_range(-1., 1.));
  safe_normalize(dir * walker.weight)
}
