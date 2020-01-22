extern crate rltk;
use rltk::{Rltk, VirtualKeyCode, Point};
extern crate specs;
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, Viewshed, CombatStats, State, Map, RunState, WantsToMelee, Item, GameLog, WantsToPickup};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_melee = ecs.write_storage::<WantsToMelee>();
    
    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);
        //attacking
        for potential_target in map.tile_content[dest_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_melee.insert(entity, WantsToMelee{ target: *potential_target })
                    .expect("Add target failed");
                return;
            }
        }
        //moving
        if !map.blocked[dest_idx] {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub const INV_WIDTH : i32 = 10;
pub const INV_HEIGHT: i32 =  4;

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();
    
    let mut target_item : Option<Entity> = None;
    for (item_entity, _item, pos) in (&entities, &items, &positions).join() {
        if pos.x == player_pos.x && pos.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }
    match target_item {
        None => gamelog.entries.insert(0, "There is nothing here to pickup.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickup>();
            pickup.insert(*player_entity, WantsToPickup{ who:*player_entity, what:item })
                .expect("Unable to insert want_pickup");
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitInput}
        Some(key) => match key {
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            //open menus
            VirtualKeyCode::C => return RunState::ShowCharacter,
            VirtualKeyCode::I => return RunState::InventoryUse,
            VirtualKeyCode::D => return RunState::InventoryDrop,
            //movement
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player( 1, 0, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player( 0,-1, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player( 0, 1, &mut gs.ecs),
            VirtualKeyCode::Y => try_move_player(-1,-1, &mut gs.ecs),
            VirtualKeyCode::U => try_move_player( 1,-1, &mut gs.ecs),
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::N => try_move_player( 1, 1, &mut gs.ecs),
            _ => { return RunState::AwaitInput}
        }
    }
    RunState::PlayerTurn
}

