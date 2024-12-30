use bevy::prelude::*;

use crate::{
    animation::tile::{TileAnimation, TileAnimationState},
    collision::collidable::Collidable,
    entities::{
        player::input::{InputAction, PlayerMouseInput},
        tile::{emit_current::CurrentMouseTileEvent, Tile, TileType, WALL_COLOR},
    },
};

#[derive(Event)]
pub struct WallEvent {
    pub tile_id: usize,
    pub action: WallAction,
}

#[derive(PartialEq)]
pub enum WallAction {
    Added,
    Removed,
}

pub struct WallManagerPlugin;

impl Plugin for WallManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WallEvent>()
            .add_systems(FixedUpdate, manage_wall_placement);
    }
}

fn manage_wall_placement(
    mut q_tiles: Query<&Tile>,
    mut current_mouse_tile_reader: EventReader<CurrentMouseTileEvent>,
    mut player_mouse_input_reader: EventReader<PlayerMouseInput>,
    mut wall_event_writer: EventWriter<WallEvent>,
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
            for tile in &mut q_tiles {
                if tile.id == current_tile && tile.tile_type != TileType::Wall {
                    wall_event_writer.send(WallEvent {
                        tile_id: tile.id,
                        action: WallAction::Added,
                    });
                }
            }
        }

        if *right_pressed {
            for tile in &mut q_tiles {
                if tile.id == current_tile && tile.tile_type == TileType::Wall {
                    wall_event_writer.send(WallEvent {
                        tile_id: tile.id,
                        action: WallAction::Removed,
                    });
                }
            }
        }
    }
}
