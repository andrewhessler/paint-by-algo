use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use bevy::prelude::*;

use crate::entities::tile::{Tile, TEMP_TILE_COLOR_1};

use super::{
    emit_current_tile::CurrentTileEvent,
    emit_pathfinding::{PathfindingEvent, PathfindingEventType},
};

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 5.0;
const PATHFINDING_ANIMATION_DELAY_MS: u64 = 25;
const PATHFINDING_TILE_BATCH: u64 = 5;

pub struct TileAnimationPlugin;

#[derive(Component, Default)]
pub struct TileAnimation {
    pub enabled: bool,
    pub growing: bool,
    pub shrinking: bool,
    pub initiated: bool,
    pub update_color: bool,
    pub color: usize,
    pub ran: bool,
    pub last_event: u32,
}

impl Plugin for TileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pathfinding_animation_timer)
            .add_systems(
                FixedUpdate,
                (
                    animate_tile,
                    initiate_animation_by_current_tile,
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
    for (mut xf, mut animate_state, mut vis, mesh) in &mut tiles {
        if animate_state.initiated && !animate_state.ran && animate_state.enabled {
            if let Some(material) = materials.get_mut(&mesh.0) {
                if animate_state.update_color {
                    let color = (animate_state.color as f32);
                    material.color = Color::hsl(color, 0.30, 0.73);
                }
            }
            if *vis == Visibility::Hidden {
                if let Some(material) = materials.get_mut(&mesh.0) {
                    material.color = TEMP_TILE_COLOR_1
                }
                vis.toggle_visible_hidden();
            }
            if !animate_state.shrinking {
                animate_state.growing = true;
            }

            if animate_state.growing {
                xf.scale += TILE_ANIMATION_STEP * time.delta_secs();
            }

            if animate_state.shrinking {
                xf.scale -= TILE_ANIMATION_STEP * time.delta_secs();
            }

            if xf.scale.y > TILE_ANIMATION_MAX_SCALE {
                animate_state.growing = false;
                animate_state.shrinking = true;
            }

            if xf.scale.y < 1. {
                animate_state.shrinking = false;
                animate_state.ran = true;
                xf.scale = Vec3::new(1., 1., 1.);
                // if *vis == Visibility::Visible {
                //     vis.toggle_visible_hidden();
                // }
            }
        }
    }
}

fn initiate_animation_by_current_tile(
    mut anim_states: Query<(&Tile, &mut TileAnimation)>,
    mut tile_activated_reader: EventReader<CurrentTileEvent>,
) {
    for event in tile_activated_reader.read() {
        for (tile, mut anim_state) in &mut anim_states {
            if tile.id == event.id {
                if anim_state.ran == false {
                    anim_state.initiated = true;
                }
            } else {
                if anim_state.ran == true {
                    anim_state.initiated = false;
                    anim_state.ran = false;
                }
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
    pub event: PathfindingEvent,
    pub color: usize,
}

fn setup_pathfinding_animation_timer(mut commands: Commands) {
    commands.insert_resource(PathfindingAnimationGate {
        timer: Timer::new(
            Duration::from_millis(PATHFINDING_ANIMATION_DELAY_MS),
            TimerMode::Repeating,
        ),
        event_queues: Vec::new(),
    });
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_calc_number() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn read_calc_number() -> usize {
    COUNTER.load(Ordering::SeqCst)
}

fn initiate_animation_by_pathfound_tile(
    mut anim_states: Query<(&Tile, &mut TileAnimation)>,
    mut pathfinding_event_reader: EventReader<PathfindingEvent>,
    time: Res<Time>,
    mut animation_gate: ResMut<PathfindingAnimationGate>,
) {
    let mut new_animation = VecDeque::default();
    let color = read_calc_number();
    for event in pathfinding_event_reader.read() {
        new_animation.push_back(AnimationFromPathfinding {
            event: event.clone(),
            color,
        });
    }

    animation_gate.event_queues.push(new_animation);
    animation_gate.timer.tick(time.delta());

    if animation_gate.timer.finished() {
        get_calc_number();
        for event_queue in &mut animation_gate.event_queues {
            for _ in 0..PATHFINDING_TILE_BATCH {
                if let Some(event) = event_queue.pop_front() {
                    let last_event: u32 = match event.event.event_type {
                        PathfindingEventType::Visited => 0,
                        PathfindingEventType::Checked => 1,
                    };
                    for (tile, mut anim_state) in &mut anim_states {
                        if tile.id == event.event.tile_id {
                            if anim_state.ran == false {
                                anim_state.update_color = true;
                                anim_state.color = event.color;
                                anim_state.initiated = true;
                                anim_state.last_event = last_event;
                            }
                        } else {
                            if anim_state.ran == true {
                                anim_state.ran = false;
                                anim_state.initiated = false;
                            }
                        }
                    }
                }
            }
        }
    }
}
