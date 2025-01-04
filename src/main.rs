use animation::highlight_cursor_tile::HighlightCursorTilePlugin;
use animation::tile::TileAnimationPlugin;
use bevy::prelude::*;
use collision::collidable::CollidablePlugin;
use current_tile::emitter::EmitCurrentTilePlugin;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::movement::PlayerMovementPlugin;
use entities::player::PlayerPlugin;
use entities::tile::TilePlugin;
use input::InputPlugin;
use pathfinding::emit_pathfinding::EmitPathfindingPlugin;
use terrain::tile_modifier::TileModifierPlugin;

mod animation {
    pub mod highlight_cursor_tile;
    pub mod tile;
}
mod collision {
    pub mod collidable;
}
mod current_tile {
    pub mod emitter;
}
mod debug;
mod entities {
    pub mod camera;
    pub mod ground;
    pub mod player;
    pub mod tile;
}
mod input;
mod pathfinding {
    pub mod algorithms;
    pub mod emit_pathfinding;
}
mod terrain {
    pub mod algorithms;
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
        .add_plugins((
            CollidablePlugin,
            EmitCurrentTilePlugin,
            EmitPathfindingPlugin,
            HighlightCursorTilePlugin,
            InputPlugin,
            PlayerMovementPlugin,
            TileAnimationPlugin,
            TileModifierPlugin,
        ))
        // .add_plugins(DebugPlugin)
        // .add_systems(PostStartup, print_entities)
        .run();
}

// use debug::DebugPlugin;
// fn print_entities(world: &World, query: Query<Entity>) {
//     for entity in &query {
//         let components = world.inspect_entity(entity);
//         for component in components {
//             println!("{:?}", component);
//         }
//     }
// }
