use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use bevy::prelude::*;

use crate::{
    entities::{
        player::input::{InputAction, PlayerInput},
        tile::{
            emit_current::CurrentTileEvent, EndUpdatedEvent, Tile, TileType, COL_COUNT,
            END_TILE_COLOR, ROW_COUNT, WALL_COLOR,
        },
    },
    pathfinding::emit_pathfinding::{PathEvent, PathfindingEvent, PathfindingNode},
    terrain::tile_modifier::{BuildType, TerrainAction, TerrainEvent, TerrainGenerationEvent},
};

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;
const PATHFINDING_ANIMATION_DELAY_MS: u64 = 1;
const PATHFINDING_TILE_BATCH: u64 = 8;

pub struct TileAnimationPlugin;

#[derive(Component, Default)]
pub struct TileAnimation {
    pub state: TileAnimationState,
    pub update_color: bool,
    pub color: usize,
    pub super_color: Option<Color>,
}

#[derive(PartialEq)]
pub enum TileAnimationState {
    Disabled,
    Idle,
    Initiated,
    Growing,
    Shrinking,
    Ran,
}

impl Default for TileAnimationState {
    fn default() -> Self {
        Self::Idle
    }
}

impl Plugin for TileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_animation_timers)
            .add_systems(
                FixedUpdate,
                (
                    animate_tile,
                    set_terrain_animation_speed_from_player_input,
                    // initiate_wall_bump_tile_animation,
                    handle_terrain_event,
                    // initiate_animation_by_pathfound_tile,
                ),
            )
            .add_systems(Update, initiate_animation_by_pathfound_tile);
    }
}

fn animate_tile(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tiles: Query<(
        &mut Transform,
        &mut TileAnimation,
        &mut Visibility,
        &MeshMaterial2d<ColorMaterial>,
    )>,
) {
    for (mut xf, mut anim, mut vis, mesh) in &mut tiles {
        if anim.state == TileAnimationState::Initiated
            || anim.state == TileAnimationState::Growing
            || anim.state == TileAnimationState::Shrinking
        {
            if let Some(material) = materials.get_mut(&mesh.0) {
                if anim.update_color {
                    let color = anim.color as f32;
                    material.color = anim.super_color.unwrap_or(Color::hsl(color, 0.30, 0.73));
                }
            }
            if *vis == Visibility::Hidden {
                if let Some(material) = materials.get_mut(&mesh.0) {
                    material.color = Color::BLACK;
                }
                *vis = Visibility::Visible;
            }
            if anim.state == TileAnimationState::Initiated {
                anim.state = TileAnimationState::Growing;
            }

            if anim.state == TileAnimationState::Growing {
                xf.scale += TILE_ANIMATION_STEP * time.delta_secs();
            }

            if anim.state == TileAnimationState::Shrinking {
                xf.scale -= TILE_ANIMATION_STEP * time.delta_secs();
            }

            if xf.scale.y > TILE_ANIMATION_MAX_SCALE {
                anim.state = TileAnimationState::Shrinking;
            }

            if xf.scale.y < 1. {
                anim.state = TileAnimationState::Ran;
                xf.scale = Vec3::new(1., 1., 1.);
                // if *vis == Visibility::Visible {
                //     vis.toggle_visible_hidden();
                // }
            }
        }
    }
}

#[derive(Resource)]
struct PathfindingAnimationGate {
    pub timer: Timer,
    pub event_queues: Vec<VecDeque<AnimationFromPathfinding>>,
}

pub struct AnimationFromPathfinding {
    pub event: PathfindingNode,
    pub color: usize,
}

fn setup_animation_timers(mut commands: Commands) {
    commands.insert_resource(PathfindingAnimationGate {
        timer: Timer::new(
            Duration::from_millis(PATHFINDING_ANIMATION_DELAY_MS),
            TimerMode::Repeating,
        ),
        event_queues: Vec::new(),
    });
    commands.insert_resource(TerrainAnimationGate {
        timer: Timer::new(
            Duration::from_millis(PATHFINDING_ANIMATION_DELAY_MS),
            TimerMode::Repeating,
        ),
        event_queues: vec![],
        fast: false,
    });
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_calc_number() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn initiate_animation_by_pathfound_tile(
    mut anim_states: Query<(&Tile, &mut TileAnimation)>,
    mut pathfinding_event_reader: EventReader<PathfindingEvent>,
    mut path_event_reader: EventReader<PathEvent>,
    time: Res<Time>,
    mut animation_gate: ResMut<PathfindingAnimationGate>,
) {
    let mut new_animation = VecDeque::default();
    let color = get_calc_number();
    for event in pathfinding_event_reader.read() {
        for node in &event.visited {
            new_animation.push_back(AnimationFromPathfinding {
                event: node.clone(),
                color,
            });
        }
    }
    for event in path_event_reader.read() {
        for node in &event.nodes {
            new_animation.push_back(AnimationFromPathfinding {
                event: node.clone(),
                color,
            })
        }
    }

    if new_animation.len() != 0 {
        animation_gate.event_queues.push(new_animation);
    }
    animation_gate.timer.tick(time.delta());

    if animation_gate.timer.finished() {
        get_calc_number();
        for event_queue in &mut animation_gate.event_queues {
            for _ in 0..PATHFINDING_TILE_BATCH {
                if let Some(event) = event_queue.pop_front() {
                    for (tile, mut anim) in &mut anim_states {
                        if tile.id == event.event.tile_id {
                            anim.update_color = true;
                            anim.color = event.color;
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

#[derive(Resource)]
struct TerrainAnimationGate {
    pub timer: Timer,
    pub event_queues: Vec<EventQueueWithTimesFired>,
    pub fast: bool,
}

struct EventQueueWithTimesFired {
    pub event_queue: VecDeque<AnimationFromTerrain>,
    pub times_fired: usize,
}

pub struct AnimationFromTerrain {
    pub event: TerrainEvent,
    pub is_wall: bool,
}

fn handle_terrain_event(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut terrain_gen_reader: EventReader<TerrainGenerationEvent>,
    mut end_reader: EventReader<EndUpdatedEvent>,
    mut q_tiles: Query<(
        &Tile,
        &mut TileAnimation,
        &MeshMaterial2d<ColorMaterial>,
        &mut Visibility,
    )>,
    time: Res<Time>,
    mut animation_gate: ResMut<TerrainAnimationGate>,
) {
    let mut new_animation = VecDeque::default();
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

    for events in terrain_gen_reader.read() {
        for event in events.terrain_events.clone() {
            for (tile, _anim, _mesh, _vis) in &mut q_tiles {
                if event.action == TerrainAction::Added {
                    if event.tile_id == tile.id {
                        new_animation.push_front(AnimationFromTerrain {
                            event: event.clone(),
                            is_wall: true,
                        });
                    }
                }
                if event.action == TerrainAction::Removed {
                    if event.tile_id == tile.id {
                        new_animation.push_front(AnimationFromTerrain {
                            event: event.clone(),
                            is_wall: false,
                        })
                    }
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
    animation_gate.timer.tick(time.delta());

    if animation_gate.timer.finished() {
        let is_fast = animation_gate.fast;

        for event_queue in &mut animation_gate.event_queues {
            let range = if event_queue.times_fired < 1 {
                event_queue.times_fired += 1;
                ROW_COUNT * COL_COUNT
            } else {
                if is_fast {
                    ROW_COUNT * COL_COUNT
                } else {
                    1
                }
            };

            for _ in 0..range {
                if let Some(event) = event_queue.event_queue.pop_back() {
                    for (tile, mut anim, _mesh, _vis) in &mut q_tiles {
                        if tile.id == event.event.tile_id {
                            anim.update_color = true;
                            if event.is_wall {
                                anim.super_color = Some(WALL_COLOR);
                            } else {
                                anim.super_color = None;
                            }
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

fn set_terrain_animation_speed_from_player_input(
    mut player_input_reader: EventReader<PlayerInput>,
    mut terrain_animation_gate: ResMut<TerrainAnimationGate>,
) {
    for player_input in player_input_reader.read() {
        if player_input.action == InputAction::Pressed && player_input.key == KeyCode::KeyM {
            terrain_animation_gate.fast = !terrain_animation_gate.fast;
        }
    }
}
