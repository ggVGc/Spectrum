use macroquad::prelude::*;

const SPECK_COUNT: i32 = 600;
const BACKGROUND_COLOR: Color = Color::new(49.0 / 256.0, 153.0 / 256.0, 158.0 / 256.0, 1.0);
const SPECK_SIZE: f32 = 10.0;
const HALF_CANVAS_SIZE: f32 = SPECK_SIZE * 20.0;
const NEIGHBOUR_DISTANCE: f32 = 2.0 * SPECK_SIZE;

fn rand_color() -> Color {
  let r: f32 = rand::gen_range(0.0, 1.0);
  let g: f32 = rand::gen_range(0.0, 1.0);
  let b: f32 = rand::gen_range(0.0, 1.0);
  Color::new(r, g, b, 1.0)
}

#[macroquad::main("Spectrum")]
async fn main() {
  let colors: Vec<Color> = vec![rand_color(), rand_color(), rand_color()];

  let mut specks: Vec<Speck> = (0..SPECK_COUNT)
    .map(|index| Speck {
      id: index,
      color: colors[rand::gen_range(0, colors.len())],
      pos: vec2(
        rand::gen_range(-HALF_CANVAS_SIZE, HALF_CANVAS_SIZE),
        rand::gen_range(-HALF_CANVAS_SIZE, HALF_CANVAS_SIZE),
      ),
      dir: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize(),
    })
    .collect();

  println!("{}, {}", screen_width(), screen_height());

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
        match update {
          Some(update) => {
            apply_update(speck, update);
            constrain_to_canvas(speck);
          }
          None => (),
        }

        draw_circle(
          center_x + speck.pos.x,
          center_y + speck.pos.y,
          SPECK_SIZE,
          speck.color,
        );
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
  let count = neighbours.len();
  if count > 0 {
    let ind = rand::gen_range(0, count);
    Some(SpeckUpdate::ChangeDir(
      (neighbours[ind].pos - speck.pos).normalize(),
    ))
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
      speck.pos += *dir;
    }
  }
}

struct Speck {
  id: i32,
  color: Color,
  pos: Vec2,
  dir: Vec2,
}

fn constrain_to_canvas(speck: &mut Speck) {
  if speck.pos.y > HALF_CANVAS_SIZE {
    // speck.dir.y = -1.0;
    speck.pos.y = HALF_CANVAS_SIZE;
  }

  if speck.pos.y < -HALF_CANVAS_SIZE {
    // speck.dir.y = 1.0;
    speck.pos.y = -HALF_CANVAS_SIZE;
  }

  if speck.pos.x < -HALF_CANVAS_SIZE {
    // speck.dir.x = 1.0;
    speck.pos.x = -HALF_CANVAS_SIZE;
  }

  if speck.pos.x > HALF_CANVAS_SIZE {
    // speck.dir.x = -1.0;
    speck.pos.x = HALF_CANVAS_SIZE;
  }
}
