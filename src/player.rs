extern crate rltk;
use rltk::{Rltk, VirtualKeyCode};
extern crate specs;
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, Viewshed, TileType, State, Map};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();
    
    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);
        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player( 1, 0, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player( 0,-1, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player( 0, 1, &mut gs.ecs),
            VirtualKeyCode::Y => try_move_player(-1,-1, &mut gs.ecs),
            VirtualKeyCode::U => try_move_player( 1,-1, &mut gs.ecs),
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::N => try_move_player( 1, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

