extern crate rltk;
use rltk::{Console, GameState, Rltk, RGB, Point};
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
mod visibility_system;
mod mob_ai_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
pub use components::*;
pub use gui::draw_ui;
pub use gamelog::GameLog;
pub use map::*;
pub use player::*;
pub use rect::Rect;
pub use visibility_system::VisibilitySystem;
pub use mob_ai_system::MobAI;
pub use map_indexing_system::MapIndexingSystem;
pub use melee_combat_system::MeleeCombatSystem;
pub use damage_system::DamageSystem;

rltk::add_wasm_support!();

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Init, AwaitInput, PlayerTurn, AITurn }

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
        vis.run_now(&self.ecs);
        mob.run_now(&self.ecs);
        idx.run_now(&self.ecs);
        mel.run_now(&self.ecs);
        dmg.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::Init => {
                self.run_systems();
                newrunstate = RunState::AwaitInput;
            }
            RunState::AwaitInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::AITurn;
            }
            RunState::AITurn => {
                self.run_systems();
                newrunstate = RunState::AwaitInput;
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph) }
        }

        draw_ui(&self.ecs, ctx);
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

    gs.ecs.register::<Player>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Mob>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Blocker>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let player_entity = gs.ecs.create_entity()
        .with(Player{})
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, attack: 5, defense: 2 })
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i,room) in map.rooms.iter().skip(1).enumerate() {
        let (x,y) = room.center();
        let glyph : u8;
        let name : String;
        let roll = rng.roll_dice(1,2);
        match roll {
            1 => { glyph = rltk::to_cp437('g') ; name = "Goblin".to_string(); }
            _ => { glyph = rltk::to_cp437('o') ; name = "Orc".to_string(); }
        }
        gs.ecs
            .create_entity()
            .with(Position{ x,y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true })
            .with(Mob{})
            .with(Name{ name: format!("{} #{}", &name, i) })
            .with(Blocker{})
            .with(CombatStats{ max_hp: 15, hp: 15, attack: 4, defense: 1 })
            .build();
    }

    gs.ecs.insert(RunState::Init);
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to RustRL".to_string()] });
    gs.ecs.insert(map);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs);
}
