use bevy::prelude::*;

use crate::entities::{
    player::input::{InputAction, PlayerInput, PlayerMouseInput},
    tile::{emit_current::CurrentMouseTileEvent, Tile, TileType, COL_COUNT, ROW_COUNT},
};

use super::algorithms::{
    wilsons::setup_and_run_wilsons, wilsons_bounded::setup_and_run_wilsons_bounded,
    TerrainAlgorithm,
};

#[derive(Clone)]
pub struct TerrainEvent {
    pub tile_id: usize,
    pub build_type: BuildType,
    pub action: TerrainAction,
}

#[derive(Event, Clone)]
pub struct TerrainGenerationEvent {
    pub terrain_events: Vec<TerrainEvent>,
}

#[derive(PartialEq, Clone)]
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
        app.add_event::<TerrainGenerationEvent>()
            .insert_resource(BuildType::Wall)
            .insert_resource(TerrainAlgorithm::WilsonsBounded)
            .add_systems(
                FixedUpdate,
                (
                    build_maze_with_algorithm,
                    fill_with_walls,
                    manage_wall_placement,
                    manage_build_type,
                    build_walls_to_block_world_wrap,
                    set_algorithm_from_key_input,
                ),
            );
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

fn build_walls_to_block_world_wrap(
    q_tiles: Query<&Tile>,
    mut player_input_reader: EventReader<PlayerInput>,
    mut terrain_gen_writer: EventWriter<TerrainGenerationEvent>,
    mut wrapping_wall_active: Local<bool>,
) {
    for input in player_input_reader.read() {
        if input.action == InputAction::Pressed && input.key == KeyCode::KeyZ {
            let action = if *wrapping_wall_active {
                *wrapping_wall_active = false;
                TerrainAction::Removed
            } else {
                *wrapping_wall_active = true;
                TerrainAction::Added
            };
            let mut walls = vec![];
            for tile in &q_tiles {
                if tile.row >= ROW_COUNT - 2
                    || tile.row < 2
                    || tile.col >= COL_COUNT - 2
                    || tile.col < 2
                {
                    walls.push(TerrainEvent {
                        tile_id: tile.id,
                        build_type: BuildType::Wall,
                        action: action.clone(),
                    });
                    terrain_gen_writer.send(TerrainGenerationEvent {
                        terrain_events: walls.clone(),
                    });
                }
            }
        }
    }
}

fn fill_with_walls(
    q_tiles: Query<&Tile>,
    mut player_input_reader: EventReader<PlayerInput>,
    mut terrain_gen_writer: EventWriter<TerrainGenerationEvent>,
) {
    for input in player_input_reader.read() {
        if input.action == InputAction::Pressed && input.key == KeyCode::KeyF {
            let tiles: Vec<&Tile> = q_tiles.iter().collect();
            let mut walls = vec![];
            for tile in tiles {
                walls.push(TerrainEvent {
                    tile_id: tile.id,
                    action: TerrainAction::Added,
                    build_type: BuildType::Wall,
                });
            }
            terrain_gen_writer.send(TerrainGenerationEvent {
                terrain_events: walls.clone(),
            });
        }
    }
}

fn build_maze_with_algorithm(
    q_tiles: Query<&Tile>,
    mut player_input_reader: EventReader<PlayerInput>,
    mut maze_gen_writer: EventWriter<TerrainGenerationEvent>,
    algo: Res<TerrainAlgorithm>,
) {
    for input in player_input_reader.read() {
        if input.action == InputAction::Pressed && input.key == KeyCode::KeyN {
            let tiles: Vec<&Tile> = q_tiles.iter().collect();
            let events = match *algo {
                TerrainAlgorithm::Wilsons => setup_and_run_wilsons(&tiles),
                TerrainAlgorithm::WilsonsBounded => setup_and_run_wilsons_bounded(&tiles),
            };
            maze_gen_writer.send(TerrainGenerationEvent {
                terrain_events: events,
            });
        }
    }
}

fn set_algorithm_from_key_input(
    mut algo: ResMut<TerrainAlgorithm>,
    mut player_input_reader: EventReader<PlayerInput>,
) {
    for event in player_input_reader.read() {
        if event.action == InputAction::Pressed {
            if event.key == KeyCode::Digit1 {
                *algo = TerrainAlgorithm::WilsonsBounded;
            }

            if event.key == KeyCode::Digit2 {
                *algo = TerrainAlgorithm::Wilsons;
            }

            if event.key == KeyCode::Digit3 {
                *algo = TerrainAlgorithm::Wilsons;
            }
        }
    }
}

fn manage_wall_placement(
    q_tiles: Query<&Tile>,
    build_state: Res<BuildType>,
    mut current_mouse_tile_reader: EventReader<CurrentMouseTileEvent>,
    mut player_mouse_input_reader: EventReader<PlayerMouseInput>,
    mut terrain_gen_writer: EventWriter<TerrainGenerationEvent>,
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
                    terrain_gen_writer.send(TerrainGenerationEvent {
                        terrain_events: vec![TerrainEvent {
                            tile_id: tile.id,
                            build_type: *build_state,
                            action: TerrainAction::Added,
                        }],
                    });
                }
            }
        }

        if *right_pressed {
            for tile in &q_tiles {
                if tile.id == current_tile {
                    terrain_gen_writer.send(TerrainGenerationEvent {
                        terrain_events: vec![TerrainEvent {
                            tile_id: tile.id,
                            build_type: *build_state,
                            action: TerrainAction::Removed,
                        }],
                    });
                }
            }
        }
    }
}
