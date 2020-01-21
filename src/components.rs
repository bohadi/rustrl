use specs::prelude::*;
use rltk::{RGB};

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component, Debug)]
pub struct Mob{}

#[derive(Component, Debug)]
pub struct Name{
    pub name : String
}

#[derive(Component, Debug)]
pub struct Blocker {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp  : i32,
    pub hp      : i32,
    pub attack  : i32,
    pub defense : i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount : i32
}