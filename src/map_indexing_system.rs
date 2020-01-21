extern crate specs;
use specs::prelude::*;
use super::{Map, Position, Blocker};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Blocker>,
                        Entities<'a>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.block_terrain();
        map.clear_tile_content();
        for (entity, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);
            let  _p : Option<&Blocker> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }
            map.tile_content[idx].push(entity);
        }
    }
}
