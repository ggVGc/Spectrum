use macroquad::prelude::*;

const SPECK_COUNT: i32 = 1000;
const SPECK_SIZE: f32 = 20.0;
const BACKGROUND_COLOR: Color = Color::new(49.0 / 256.0, 153.0 / 256.0, 158.0 / 256.0, 1.0);

#[macroquad::main("Spectrum")]
async fn main() {
  let mut specks: Vec<Speck> = (0..SPECK_COUNT)
    .map(|index| {
      let r: f32 = rand::gen_range(0.0, 1.0);
      let g: f32 = rand::gen_range(0.0, 1.0);
      let b: f32 = rand::gen_range(0.0, 1.0);
      Speck {
        id: index,
        color: Color::new(r, g, b, 1.0),
        pos: vec2(
          rand::gen_range(0.0, screen_width()),
          rand::gen_range(0.0, screen_height()),
        ),
      }
    })
    .collect();

  let neighbour_distance = 2.0 * screen_height() / SPECK_SIZE;

  loop {
    clear_background(BACKGROUND_COLOR);

    let new_specks: Vec<Speck> = specks
      .iter()
      .map(|speck| {
        draw_circle(speck.pos.x, speck.pos.y, SPECK_SIZE, speck.color);
        let neighbours: Vec<&Speck> = specks
          .iter()
          .filter(|other| {
            let d2 = other.pos.distance_squared(speck.pos);
            d2 < neighbour_distance * neighbour_distance && speck.id != other.id
          })
          .collect();

        update_speck(speck, neighbours)
      })
      .collect();

    specks = new_specks;
    next_frame().await
  }
}

fn update_speck(old_speck: &Speck, neighbours: Vec<&Speck>) -> Speck {
  let mut speck = old_speck.clone();

  let count = neighbours.len();
  let dir = if count > 0 {
    let ind = rand::gen_range(0, count);
    neighbours[ind].pos - speck.pos
  } else {
    vec2(rand::gen_range(0.0, 1.0), rand::gen_range(0.0, 1.0))
  }
  .normalize();

  speck.pos += dir;

  speck
}

#[derive(Copy, Clone)]
struct Speck {
  id: i32,
  color: Color,
  pos: Vec2,
}
