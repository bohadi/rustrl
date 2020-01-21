extern crate specs;
use specs::prelude::*;
use super::{CombatStats, WantsToMelee, Name, SufferDamage};
use rltk::console;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToMelee>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>,
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut suffers_dmg) = data;
        for (_e, wants_melee, name, stats) in (&entities, &wants_melee, &names, &combat_stats).join() {
            let target_stats = combat_stats.get(wants_melee.target).unwrap();
            let target_name = names.get(wants_melee.target).unwrap();
            if stats.hp > 0 && target_stats.hp > 0 {
                let dmg = i32::max(0, stats.attack - target_stats.defense);
                if dmg == 0 {
                    console::log(&format!("{} is unable to hurt {}", &name.name, &target_name.name));
                } else {
                    console::log(&format!("{} hits {}, for {} hp", &name.name, &target_name.name, dmg));
                    suffers_dmg.insert(wants_melee.target, SufferDamage{ amount: dmg }).expect("Unable to do damage");
                }
            }
        }
        wants_melee.clear();
    }
}
