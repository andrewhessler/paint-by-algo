use bevy::prelude::*;

use super::{TileAnimation, TileAnimationState};
use crate::input::{InputAction, KeyboardInputEvent};
use crate::{
    entities::tile::{EndUpdatedEvent, Tile, COL_COUNT, END_TILE_COLOR, ROW_COUNT, WALL_COLOR},
    terrain::tile_modifier::{TerrainAction, TerrainEvent, TerrainGenerationEvent},
};
use std::{collections::VecDeque, time::Duration};

const TERRAIN_ANIMATION_DELAY_MS: u64 = 1;
const TERRAIN_TILE_BATCH: u64 = 1;

pub struct TerrainTileAnimationPlugin;

impl Plugin for TerrainTileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainAnimationGate {
            timer: Timer::new(
                Duration::from_millis(TERRAIN_ANIMATION_DELAY_MS),
                TimerMode::Repeating,
            ),
            event_queues: vec![],
            fast_mode_enabled: false,
        })
        .add_systems(
            FixedUpdate,
            (
                initiate_animation,
                handle_terrain_event,
                handle_end_event,
                set_terrain_animation_speed_from_keyboard_input,
            ),
        );
    }
}

#[derive(Resource)]
struct TerrainAnimationGate {
    timer: Timer,
    event_queues: Vec<EventQueueWithTimesFired>,
    fast_mode_enabled: bool,
}

struct EventQueueWithTimesFired {
    event_queue: VecDeque<AnimationFromTerrain>,
    times_fired: usize,
}

struct AnimationFromTerrain {
    event: TerrainEvent,
    is_wall: bool,
}

fn initiate_animation(
    time: Res<Time>,
    mut animation_gate: ResMut<TerrainAnimationGate>,
    mut q_tiles: Query<(
        &Tile,
        &mut TileAnimation,
        &MeshMaterial2d<ColorMaterial>,
        &mut Visibility,
    )>,
) {
    animation_gate.timer.tick(time.delta());

    if animation_gate.timer.finished() {
        let is_fast = animation_gate.fast_mode_enabled;

        for event_queue in &mut animation_gate.event_queues {
            let range = if event_queue.times_fired < 1 {
                event_queue.times_fired += 1;
                ROW_COUNT * COL_COUNT
            } else {
                if is_fast {
                    ROW_COUNT * COL_COUNT
                } else {
                    TERRAIN_TILE_BATCH as usize
                }
            };

            for _ in 0..range {
                if let Some(event) = event_queue.event_queue.pop_back() {
                    for (tile, mut anim, _mesh, _vis) in &mut q_tiles {
                        if tile.id == event.event.tile_id {
                            anim.update_color = true;
                            anim.super_color = if event.is_wall {
                                Some(WALL_COLOR)
                            } else {
                                None
                            };
                            if anim.state == TileAnimationState::Ran {
                                anim.state = TileAnimationState::Initiated;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_terrain_event(
    mut animation_gate: ResMut<TerrainAnimationGate>,
    mut terrain_gen_reader: EventReader<TerrainGenerationEvent>,
    q_tiles: Query<&Tile>,
) {
    let mut new_animation = VecDeque::default();

    for events in terrain_gen_reader.read() {
        for event in events.terrain_events.clone() {
            for tile in &q_tiles {
                if event.tile_id == tile.id {
                    let is_wall = event.action == TerrainAction::Added;
                    new_animation.push_front(AnimationFromTerrain {
                        event: event.clone(),
                        is_wall,
                    });
                }
            }
        }
    }

    if new_animation.len() != 0 {
        animation_gate.event_queues.push(EventQueueWithTimesFired {
            event_queue: new_animation,
            times_fired: 0,
        });
    }
}

fn handle_end_event(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut end_reader: EventReader<EndUpdatedEvent>,
    mut q_tiles: Query<(
        &Tile,
        &mut TileAnimation,
        &MeshMaterial2d<ColorMaterial>,
        &mut Visibility,
    )>,
) {
    for event in end_reader.read() {
        if let Some(curr_end) = event.new_end_id {
            for (tile, mut anim, mesh, mut vis) in &mut q_tiles {
                if tile.id == curr_end {
                    anim.state = TileAnimationState::Disabled;
                    *vis = Visibility::Visible;
                    if let Some(material) = materials.get_mut(&mesh.0) {
                        material.color = END_TILE_COLOR;
                    }
                }
            }
        }
        if let Some(old_end) = event.old_end_id {
            for (tile, mut anim, _mesh, mut vis) in &mut q_tiles {
                if tile.id == old_end {
                    *vis = Visibility::Hidden;
                    anim.state = TileAnimationState::Initiated;
                }
            }
        }
    }
}

fn set_terrain_animation_speed_from_keyboard_input(
    mut keyboard_input_reader: EventReader<KeyboardInputEvent>,
    mut terrain_animation_gate: ResMut<TerrainAnimationGate>,
) {
    for keyboard_input in keyboard_input_reader.read() {
        if keyboard_input.action == InputAction::Pressed && keyboard_input.key == KeyCode::KeyM {
            terrain_animation_gate.fast_mode_enabled = !terrain_animation_gate.fast_mode_enabled;
        }
    }
}
