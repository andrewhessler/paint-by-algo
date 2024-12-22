use bevy::prelude::*;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::PlayerPlugin;
use entities::tile::TilePlugin;
use systems::emit_current_tile::EmitCurrentTilePlugin;
use systems::player_movement::PlayerMovementPlugin;
use systems::tile_animation::TileAnimationPlugin;

mod debug;
mod entities;
mod pathfinding_algorithms;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ***************************************
        // Entities
        .add_plugins((PlayerPlugin, GroundPlugin, TilePlugin, SceneCameraPlugin))
        // ***************************************
        // Systems and Interactions
        .add_plugins((
            PlayerMovementPlugin,
            TileAnimationPlugin,
            EmitCurrentTilePlugin,
        ))
        .run();
}
