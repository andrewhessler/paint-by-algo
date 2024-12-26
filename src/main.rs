use bevy::prelude::*;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::PlayerPlugin;
use entities::tile::TilePlugin;
use systems::emit_current_tile::EmitCurrentTilePlugin;
use systems::emit_pathfinding::EmitPathfindingPlugin;
use systems::player_movement::PlayerMovementPlugin;
use systems::tile_animation::TileAnimationPlugin;

mod debug;
mod entities;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ***************************************
        // Entities
        .add_plugins((PlayerPlugin, GroundPlugin, TilePlugin, SceneCameraPlugin))
        // ***************************************
        // Systems: Monitors and Actions
        .add_plugins((
            EmitCurrentTilePlugin,
            EmitPathfindingPlugin,
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
