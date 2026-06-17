#![no_std]

mod bullet;
mod common;
mod entity;
mod shooter;
mod the_lady;

use rico8::*;

use crate::{bullet::Bullet, entity::Entity, the_lady::TheLady};

#[derive(Debug)]
struct Cart {
    enemy_bullet: Option<Bullet>,
    friendly_bullet: Option<Bullet>,

    the_lady: TheLady,
}

impl Game for Cart {
    fn update(&mut self, ctx: &mut Context) {
        self.the_lady.update(ctx);

        match &mut self.enemy_bullet {
            Some(b) => {
                b.update(ctx);
                if b.outside() {
                    self.enemy_bullet = None;
                }
            }
            None => self.enemy_bullet = Some(Bullet::new_enemy(20.0, 0.0, ctx)),
        }

        match &mut self.friendly_bullet {
            Some(b) => {
                b.update(ctx);
                if b.outside() {
                    self.friendly_bullet = None;
                }
            }
            None => {
                self.friendly_bullet = Some(Bullet::new_friendly(80.0, SCREEN_H as f32 - 1.0, ctx))
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::BLACK);
        // TODO: Scrolling map.
        gfx.map(0, 0, 0.0, 0.0, 16, 32, BitFlags::empty());

        self.the_lady.draw(gfx);

        if let Some(b) = &self.enemy_bullet {
            b.draw(gfx);
        }

        if let Some(b) = &self.friendly_bullet {
            b.draw(gfx);
        }
    }
}

rico8::game!(
    Cart = Cart {
        enemy_bullet: None,
        friendly_bullet: None,

        the_lady: TheLady::new(),
    }
);
