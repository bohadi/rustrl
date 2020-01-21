extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Mob, WantsToMelee, RunState, Map, Position};
extern crate rltk;
use rltk::{Point};

pub struct MobAI {}

impl<'a> System<'a> for MobAI {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Mob>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>,
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, mob, mut position, mut wants_melee) = data;

        if *runstate != RunState::AITurn { return; }
        for (entity, mut viewshed, _m, mut pos) in (&entities, &mut viewshed, &mob, &mut position).join() {
            let dist = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x,pos.y), *player_pos);
            if dist < 1.5 {
                wants_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                //console::log(&format!("{} shouts insults!", name.name));
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map
                );
                if path.success && path.steps.len()>1 {
                    let mut idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = false;
                    pos.x = path.steps[1] % map.width;
                    pos.y = path.steps[1] / map.width;
                    idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
