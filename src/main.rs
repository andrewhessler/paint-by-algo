use animation::tile_animation::TileAnimationPlugin;
use bevy::prelude::*;
use collision::collidable::CollidablePlugin;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::player_input::PlayerInputPlugin;
use entities::player::player_movement::PlayerMovementPlugin;
use entities::player::PlayerPlugin;
use entities::tile::emit_current_tile::EmitCurrentTilePlugin;
use entities::tile::TilePlugin;
use pathfinding::emit_pathfinding::EmitPathfindingPlugin;

mod animation {
    pub mod tile_animation;
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
            PlayerInputPlugin,
            PlayerMovementPlugin,
            TileAnimationPlugin,
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
