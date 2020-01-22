extern crate specs;
use specs::prelude::*;
use super::{Item, player::INV_HEIGHT, player::INV_WIDTH, Shape, WantsToDrop, WantsToUse, WantsToPickup, BackpackOf, Name, Position, GameLog, Moongrass, CombatStats};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, WantsToPickup>,
                        ReadStorage<'a, Item>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, BackpackOf>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, items, mut positions, names, mut backpack) = data;

        for pickup in wants_pickup.join() {
            let (mut x, mut y) = (0,0);
            if pickup.who == *player_entity {
                let shape = items.get(pickup.what).unwrap().shape;

                let mut has_space = false;
                /*
                for packed in (&backpack).join().filter(|item| item.owner == pickup.who) {
                    // make index of inventory
                }
                while !has_space {
                    match shape {
                        Shape::One => has_space = true,
                        Shape::Two => has_space = true,
                        Shape::Three => has_space = true,
                        Shape::Square => has_space = true,
                        Shape::ThreeWide => has_space = true,
                        Shape::FourWide => has_space = true
                    }
                    y += 1;
                    if y > INV_HEIGHT-1 {
                        y = 0;
                        x += 1;
                        if x == INV_WIDTH { break }
                    }
                }
                */
                has_space = true;

                if has_space {
                    gamelog.entries.insert(0, format!("You pick up the {}.", names.get(pickup.what).unwrap().name));
                } else {
                    gamelog.entries.insert(0, format!("{} tries to pick up the {} but has no space.", names.get(pickup.who).unwrap().name , names.get(pickup.what).unwrap().name));
                    return;
                }
            }
            positions.remove(pickup.what);
            backpack.insert(pickup.what, BackpackOf{ owner: pickup.who, x:x,y:y })
                .expect("Unable to insert item");
        }

        wants_pickup.clear();
    }

}

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToUse>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Moongrass>,
                        WriteStorage<'a, CombatStats>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, entities, mut wants_use, names, moongrass, mut combat_stats) = data;
        for (entity, usable, stats) in (&entities, &wants_use, &mut combat_stats).join() {
            let item = moongrass.get(usable.item);
            match item {
                None => {}
                Some(item) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + item.heal_amount);
                    if entity == *player {
                        gamelog.entries.insert(0, format!("The {} invigorates you, healing {} health.", names.get(usable.item).unwrap().name, item.heal_amount));
                    }
                    entities.delete(usable.item)
                        .expect("Delete failed");
                }
            }
        }
        wants_use.clear();
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDrop>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, BackpackOf>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, entities, mut wants_drop, names, mut positions, mut backpack) = data;
        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos : Position = Position{x:0,y:0};
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions.insert(to_drop.item, Position{ x:dropper_pos.x, y:dropper_pos.y})
                .expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player {
                gamelog.entries.insert(0, format!("You drop the {}.", names.get(to_drop.item).unwrap().name));
            }
        }
        wants_drop.clear();
    }
}
