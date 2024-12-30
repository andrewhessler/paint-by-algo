use bevy::prelude::*;

use crate::entities::{
    player::input::{InputAction, PlayerInput, PlayerMouseInput},
    tile::{emit_current::CurrentMouseTileEvent, Tile, TileType},
};

#[derive(Event)]
pub struct TerrainEvent {
    pub tile_id: usize,
    pub build_type: BuildType,
    pub action: TerrainAction,
}

#[derive(PartialEq)]
pub enum TerrainAction {
    Added,
    Removed,
}

#[derive(Resource, Copy, Clone, Debug, PartialEq)]
pub enum BuildType {
    Wall,
    End,
}

pub struct TileModifierPlugin;

impl Plugin for TileModifierPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TerrainEvent>()
            .insert_resource(BuildType::Wall)
            .add_systems(FixedUpdate, (manage_wall_placement, manage_build_type));
    }
}

fn manage_build_type(
    mut build_type: ResMut<BuildType>,
    mut player_input_reader: EventReader<PlayerInput>,
) {
    for event in player_input_reader.read() {
        if event.action == InputAction::Pressed {
            if event.key == KeyCode::KeyE {
                *build_type = BuildType::End;
            }

            if event.key == KeyCode::KeyR {
                *build_type = BuildType::Wall;
            }
        }
    }
}

fn manage_wall_placement(
    q_tiles: Query<&Tile>,
    build_state: Res<BuildType>,
    mut current_mouse_tile_reader: EventReader<CurrentMouseTileEvent>,
    mut player_mouse_input_reader: EventReader<PlayerMouseInput>,
    mut wall_event_writer: EventWriter<TerrainEvent>,
    mut current_tile_id: Local<Option<usize>>,
    mut left_pressed: Local<bool>,
    mut right_pressed: Local<bool>,
) {
    for event in current_mouse_tile_reader.read() {
        *current_tile_id = event.id;
    }

    for event in player_mouse_input_reader.read() {
        if event.key == MouseButton::Left && event.action == InputAction::Pressed {
            *left_pressed = true;
        }
        if event.key == MouseButton::Left && event.action == InputAction::Released {
            *left_pressed = false;
        }

        if event.key == MouseButton::Right && event.action == InputAction::Pressed {
            *right_pressed = true;
        }
        if event.key == MouseButton::Right && event.action == InputAction::Released {
            *right_pressed = false;
        }
    }
    if let Some(current_tile) = *current_tile_id {
        if *left_pressed {
            for tile in &q_tiles {
                if tile.id == current_tile && tile.tile_type != TileType::Wall {
                    wall_event_writer.send(TerrainEvent {
                        tile_id: tile.id,
                        build_type: *build_state,
                        action: TerrainAction::Added,
                    });
                }
            }
        }

        if *right_pressed {
            for tile in &q_tiles {
                if tile.id == current_tile && tile.tile_type == TileType::Wall {
                    wall_event_writer.send(TerrainEvent {
                        tile_id: tile.id,
                        build_type: *build_state,
                        action: TerrainAction::Removed,
                    });
                }
            }
        }
    }
}
