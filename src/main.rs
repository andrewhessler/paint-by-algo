use animation::highlight_cursor_tile::HighlightCursorTilePlugin;
use animation::tile::TileAnimationPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
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
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Paint By Algo".into(),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins((PlayerPlugin, GroundPlugin, TilePlugin, SceneCameraPlugin))
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
        //        .add_plugins(DebugPlugin)
        .run();
}

// use debug::DebugPlugin;
