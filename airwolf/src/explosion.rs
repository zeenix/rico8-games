use heapless::Vec;
use rico8::{Color, Context, Graphics};

use crate::common::Position;

#[derive(Debug)]
pub struct Explosion {
    sparks: Vec<Spark, MAX_SPARKS>,
}

impl Explosion {
    pub fn new(pos: Position, ctx: &mut Context) -> Self {
        let mut sparks = Vec::new();

        for _ in 0..MAX_SPARKS {
            sparks
                .push(Spark {
                    pos: pos.clone(),
                    x_velocity: ctx.random_integer(MIN_VELOCIY..MAX_VELOCIY) as i8,
                    y_velocity: ctx.random_integer(MIN_VELOCIY..MAX_VELOCIY) as i8,
                    radius: ctx.random_integer(MIN_RADIUS..MAX_RADIUS) as u8,
                    mass: ctx.random_integer(MIN_MASS..MAX_MASS) as u8,
                })
                // We add exactly `MAX_SPARKS` sparks so this can't be `Err`.
                .unwrap();
        }

        Self { sparks }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        if self.disappeared() {
            return;
        }

        self.sparks.retain_mut(
            |Spark {
                 pos: Position { x, y },
                 radius,
                 x_velocity,
                 y_velocity,
                 mass,
             }| {
                let x_velocity = *x_velocity as f32 / i8::MAX as f32;
                let y_velocity = *y_velocity as f32 / i8::MAX as f32;
                let mass = *mass as f32 * 8.0 / u8::MAX as f32;

                *x += x_velocity / mass;
                *y += y_velocity / mass;
                *radius = radius.saturating_sub(RADIUS_SHRINK_SPEED);

                *radius > 0
            },
        );
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        self.sparks.iter().for_each(
            |Spark {
                 pos: Position { x, y },
                 radius,
                 ..
             }| {
                let radius = *radius as f32 / u8::MAX as f32 * 2.0;
                gfx.circle_fill(*x as i32, *y as i32, radius as u32, SPARK_COLOR);
            },
        );
    }

    pub fn disappeared(&self) -> bool {
        self.sparks.is_empty()
    }
}

#[derive(Debug)]
struct Spark {
    pos: Position,
    x_velocity: i8,
    y_velocity: i8,
    radius: u8,
    mass: u8,
}

const MAX_SPARKS: usize = 50;
const RADIUS_SHRINK_SPEED: u8 = u8::MAX / 15;
const SPARK_COLOR: Color = Color::LIGHT_GREY;

const MIN_VELOCIY: i32 = i8::MIN as i32;
const MAX_VELOCIY: i32 = i8::MAX as i32;
const MIN_RADIUS: i32 = (u8::MAX / 15) as i32;
const MAX_RADIUS: i32 = u8::MAX as i32;
const MIN_MASS: i32 = (u8::MAX / 2) as i32;
const MAX_MASS: i32 = u8::MAX as i32;
