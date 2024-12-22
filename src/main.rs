use bevy::prelude::*;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::PlayerPlugin;
use entities::tile::TilePlugin;
use systems::player_movement::PlayerMovementPlugin;
use systems::set_current_tile_as_activated::SetCurrentTilePlugin;
use systems::tile_animation::TileAnimationPlugin;

mod debug;
mod entities;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, GroundPlugin, TilePlugin, SceneCameraPlugin))
        .add_plugins((
            PlayerMovementPlugin,
            TileAnimationPlugin,
            SetCurrentTilePlugin,
        ))
        .run();
}
