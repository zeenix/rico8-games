use rico8::{SpriteId, SCREEN_H};

use crate::{
    common::{Direction, Position, Size, Sprite},
    entity::{self, Entity},
};

#[derive(Debug)]
pub struct Bullet {
    pos: Position,
    entity_type: entity::Type,
}

impl Bullet {
    pub fn new_friendly(pos: Position) -> Self {
        Self {
            pos,
            entity_type: entity::Type::FriendlyBullet,
        }
    }

    pub fn new_enemy(pos: Position) -> Self {
        Self {
            pos,
            entity_type: entity::Type::EnemyBullet,
        }
    }
}

impl Entity for Bullet {
    fn position(&self) -> Position {
        self.pos
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.pos
    }

    fn sprite(&self) -> Sprite {
        match self.entity_type {
            entity::Type::EnemyBullet => Sprite {
                id: ENEMY_SPRITE_ID,
                size: ENEMY_SIZE,
            },
            entity::Type::FriendlyBullet => Sprite {
                id: FRIENDLY_SPRITE_ID,
                size: FRIENDLY_SIZE,
            },
            _ => unreachable!("unknown bullet type"),
        }
    }

    fn entity_type(&self) -> entity::Type {
        self.entity_type
    }

    fn update(&mut self) -> bool {
        if self.is_enemy() {
            self.go(Some(Direction::Down), SPEED);

            self.pos.y >= SCREEN_H as f32
        } else {
            self.go(Some(Direction::Up), SPEED);

            self.pos.y < 0.0
        }
    }
}

const FRIENDLY_SPRITE_ID: SpriteId = SpriteId(64);
const FRIENDLY_SIZE: Size = Size {
    width: 6,
    height: 8,
};
const ENEMY_SPRITE_ID: SpriteId = SpriteId(65);
const ENEMY_SIZE: Size = Size {
    width: 1,
    height: 6,
};
const SPEED: u8 = 2;
