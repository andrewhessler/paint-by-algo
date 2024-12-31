use animation::highlight_cursor_tile::HighlightCursorTilePlugin;
use animation::tile::TileAnimationPlugin;
use bevy::prelude::*;
use collision::collidable::CollidablePlugin;
use debug::DebugPlugin;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::input::PlayerInputPlugin;
use entities::player::movement::PlayerMovementPlugin;
use entities::player::PlayerPlugin;
use entities::tile::emit_current::EmitCurrentTilePlugin;
use entities::tile::TilePlugin;
use pathfinding::emit_pathfinding::EmitPathfindingPlugin;
use terrain::tile_modifier::TileModifierPlugin;

mod animation {
    pub mod highlight_cursor_tile;
    pub mod tile;
}
mod collision {
    pub mod collidable;
}
mod debug;
mod entities {
    pub mod camera;
    pub mod ground;
    pub mod player;
    pub mod tile;
}
mod pathfinding {
    pub mod algorithms;
    pub mod emit_pathfinding;
}
mod terrain {
    pub mod tile_modifier;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ***************************************
        // Entities
        .add_plugins((PlayerPlugin, GroundPlugin, TilePlugin, SceneCameraPlugin))
        // ***************************************
        // Systems: Monitors and Actions
        // .add_plugins(DebugPlugin)
        .add_plugins((
            // CollidablePlugin,
            EmitCurrentTilePlugin,
            EmitPathfindingPlugin,
            HighlightCursorTilePlugin,
            PlayerInputPlugin,
            PlayerMovementPlugin,
            TileAnimationPlugin,
            TileModifierPlugin,
        ))
        // .add_systems(PostStartup, print_entities)
        .run();
}

fn print_entities(world: &World, query: Query<Entity>) {
    for entity in &query {
        let components = world.inspect_entity(entity);
        for component in components {
            println!("{:?}", component);
        }
    }
}
