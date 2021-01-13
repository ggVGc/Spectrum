mod personality;
mod speck;

use macroquad::prelude::*;

use crate::personality::*;
use crate::rand::gen_range;
use crate::speck::*;
// use chrono;
// use stdweb;
// use std::time::{SystemTime, UNIX_EPOCH};

const SPECK_COUNT: i32 = 400;
const BACKGROUND_COLOR: Color = Color::new(50.0/ 256.0, 8.0/ 256.0, 8.0 / 256.0, 1.0);
const SPECK_SIZE: f32 = 10.0;
const HALF_CANVAS_SIZE: f32 = SPECK_SIZE * 20.0;
const NEIGHBOUR_DISTANCE: f32 = 2.0 * SPECK_SIZE;
const MAX_SPEED: f32 = 2.0;
const MAX_AGE: f32 = 100.0;
const DIR_UPDATE_CYCLE: i32 = 10;

fn rand_color() -> Color {
  let r: f32 = gen_range(0.0, 1.0);
  let g: f32 = gen_range(0.0, 1.0);
  let b: f32 = gen_range(0.0, 1.0);
  Color::new(r, g, b, 1.0)
}

#[macroquad::main("Spectrum")]
async fn main() {
  let now = miniquad::date::now() as u64;
  rand::srand(now);

  let colors: Vec<Color> = vec![rand_color(), rand_color(), rand_color()];

  let mk_new_speck = |id: i32| {
    rand_speck(
      id,
      colors.len(),
      HALF_CANVAS_SIZE,
      MAX_AGE,
      DIR_UPDATE_CYCLE,
    )
  };

  let mut specks: Vec<Speck> = (0..SPECK_COUNT).map(|index| mk_new_speck(index)).collect();

  loop {
    clear_background(BACKGROUND_COLOR);

    let center_x: f32 = screen_width() / 2.0;
    let center_y: f32 = screen_height() / 2.0;

    let updates: Vec<_> = specks
      .iter()
      .map(|speck| {
        let neighbours = get_neighbours(NEIGHBOUR_DISTANCE, speck, &specks);
        get_speck_update(speck, neighbours)
      })
      .collect();

    specks
      .iter_mut()
      .zip(updates.iter())
      .for_each(|(speck, update)| {
        speck.age += 0.01;
        if speck.dir_update_counter > DIR_UPDATE_CYCLE {
          speck.dir_update_counter = 0;
        } else {
          speck.dir_update_counter += 1;
        }
        if speck.age >= MAX_AGE {
          *speck = mk_new_speck(speck.id);
          speck.age = 0.0;
        }

        draw_circle(
          center_x + speck.pos.x,
          center_y + speck.pos.y,
          SPECK_SIZE * (speck.age / MAX_AGE),
          colors[speck.color_index],
        );

        match update {
          Some(update) => {
            apply_update(speck, update);
          }
          None => (),
        }

        speck.pos += speck.dir * MAX_SPEED * speck.personality.stamina;
        constrain_to_canvas(speck);
      });

    next_frame().await
  }
}

fn get_neighbours<'a>(distance: f32, speck: &Speck, others: &'a [Speck]) -> Vec<&'a Speck> {
  others
    .iter()
    .filter(|other| {
      let d2 = other.pos.distance_squared(speck.pos);
      d2 < distance * distance && speck.id != other.id
    })
    .collect()
}

fn get_speck_update(speck: &Speck, neighbours: Vec<&Speck>) -> Option<SpeckUpdate> {
  if speck.dir_update_counter == 0 && neighbours.len() > 0 {
    Some(SpeckUpdate::ChangeDir(dir_from_personality(
      speck.pos,
      &speck.personality,
      &neighbours,
    )))
  } else {
    None
  }
}

enum SpeckUpdate {
  ChangeDir(Vec2),
}

fn apply_update(speck: &mut Speck, update: &SpeckUpdate) {
  match update {
    SpeckUpdate::ChangeDir(dir) => {
      speck.dir = *dir;
    }
  }
}

fn constrain_to_canvas(speck: &mut Speck) {
  if speck.pos.y > HALF_CANVAS_SIZE {
    speck.dir.y = -1.0;
    speck.pos.y = HALF_CANVAS_SIZE;
  }

  if speck.pos.y < -HALF_CANVAS_SIZE {
    speck.dir.y = 1.0;
    speck.pos.y = -HALF_CANVAS_SIZE;
  }

  if speck.pos.x < -HALF_CANVAS_SIZE {
    speck.dir.x = 1.0;
    speck.pos.x = -HALF_CANVAS_SIZE;
  }

  if speck.pos.x > HALF_CANVAS_SIZE {
    speck.dir.x = -1.0;
    speck.pos.x = HALF_CANVAS_SIZE;
  }
}
