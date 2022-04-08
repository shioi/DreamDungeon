pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

//randomly move component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

//health
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

//for names on top of enemies head
#[derive(Clone, PartialEq)]
pub struct Name(pub String);

//for attack

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

//chasing
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

//for fields of view
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

//for items
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvideHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvideDungeonMap;
