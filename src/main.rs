extern crate rltk;
use rltk::{Console, GameState, Rltk, Point};
extern crate specs;
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

mod components;
mod gui;
mod gamelog;
mod map;
mod player;
mod rect;
mod spawner;
mod visibility_system;
mod mob_ai_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
mod inventory_system;
pub use components::*;
pub use gui::*;
pub use gamelog::GameLog;
pub use map::*;
pub use player::*;
pub use rect::Rect;
pub use spawner::*;
pub use visibility_system::VisibilitySystem;
pub use mob_ai_system::MobAI;
pub use map_indexing_system::MapIndexingSystem;
pub use melee_combat_system::MeleeCombatSystem;
pub use damage_system::DamageSystem;
pub use inventory_system::ItemCollectionSystem;
pub use inventory_system::ItemUseSystem;
pub use inventory_system::ItemDropSystem;

rltk::add_wasm_support!();

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Init, AwaitInput, PlayerTurn, AITurn, ShowCharacter, InventoryUse, InventoryDrop }

pub struct State {
    pub ecs: World,
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        let mut mob = MobAI{};
        let mut idx = MapIndexingSystem{};
        let mut mel = MeleeCombatSystem{};
        let mut dmg = DamageSystem{};
        let mut pck = ItemCollectionSystem{};
        let mut itm = ItemUseSystem{};
        let mut drp = ItemDropSystem{};
        vis.run_now(&self.ecs);
        mob.run_now(&self.ecs);
        idx.run_now(&self.ecs);
        mel.run_now(&self.ecs);
        dmg.run_now(&self.ecs);
        pck.run_now(&self.ecs);
        itm.run_now(&self.ecs);
        drp.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        draw_map(&self.ecs, ctx);
        {
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            let map = self.ecs.fetch::<Map>();

            let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
            data.sort_by(|&a, &b| b.1.render_priority.cmp(&a.1.render_priority) );
            for (pos, render) in data.iter() {
                let idx = map.xy_idx(pos.x, pos.y);
                if map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph) }
            }
        }
        draw_ui(&self.ecs, ctx);

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::Init => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitInput;
            }
            RunState::AwaitInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AITurn;
            }
            RunState::AITurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitInput;
            }
            RunState::ShowCharacter => {
                if gui::show_character(self, ctx) == gui::MenuResult::Cancel {
                    newrunstate = RunState::AwaitInput;
                }
            }
            RunState::InventoryUse => {
                let result = gui::show_inventory("Inventory", self, ctx);
                match result.0 {
                    gui::MenuResult::Cancel => newrunstate = RunState::AwaitInput,
                    gui::MenuResult::NoResponse => {}
                    gui::MenuResult::Selected => {
                        let item = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToUse>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToUse{item:item})
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::InventoryDrop => {
                let result = gui::show_inventory("Drop what?", self, ctx);
                match result.0 {
                    gui::MenuResult::Cancel => newrunstate = RunState::AwaitInput,
                    gui::MenuResult::NoResponse => {}
                    gui::MenuResult::Selected => {
                        let item = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDrop>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDrop{item:item})
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);
    }
}

fn main() {
    let mut context = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");
    context.with_post_scanlines(true);
    let mut gs = State {
        ecs: World::new(),
    };
    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.register::<Name>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    //combat
    gs.ecs.register::<Player>();
    gs.ecs.register::<Mob>();
    gs.ecs.register::<Blocker>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    //item
    gs.ecs.register::<Item>();
    gs.ecs.register::<BackpackOf>();
    gs.ecs.register::<WantsToPickup>();
    gs.ecs.register::<WantsToDrop>();
    gs.ecs.register::<WantsToUse>();
    gs.ecs.register::<Moongrass>();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(map);
    gs.ecs.insert(RunState::Init);
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to RustRL".to_string()] });

    rltk::main_loop(context, gs);
}
