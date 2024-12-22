use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use crate::entities::tile::Tile;

use super::{emit_current_tile::CurrentTileEvent, emit_pathfinding::PathfindingEvent};

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;
const PATHFINDING_ANIMATION_DELAY_MS: u64 = 1;
const PATHFINDING_TILE_BATCH: u64 = 25;

pub struct TileAnimationPlugin;

#[derive(Component, Default)]
pub struct TileAnimation {
    pub enabled: bool,
    pub growing: bool,
    pub shrinking: bool,
    pub initiated: bool,
    pub ran: bool,
}

impl Plugin for TileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pathfinding_animation_timer)
            .add_systems(
                FixedUpdate,
                (
                    animate_tile,
                    initiate_animation_by_current_tile,
                    initiate_animation_by_pathfound_tile,
                ),
            );
    }
}

fn animate_tile(
    time: Res<Time>,
    mut tiles: Query<(&mut Transform, &mut TileAnimation, &mut Visibility)>,
) {
    for (mut xf, mut animate_state, mut vis) in &mut tiles {
        if animate_state.initiated && !animate_state.ran && animate_state.enabled {
            if *vis == Visibility::Hidden {
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
                if *vis == Visibility::Visible {
                    vis.toggle_visible_hidden();
                }
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
    pub event_queues: Vec<VecDeque<PathfindingEvent>>,
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

fn initiate_animation_by_pathfound_tile(
    mut anim_states: Query<(&Tile, &mut TileAnimation)>,
    mut tile_activated_reader: EventReader<PathfindingEvent>,
    time: Res<Time>,
    mut animation_gate: ResMut<PathfindingAnimationGate>,
) {
    let mut new_animation = VecDeque::default();
    for event in tile_activated_reader.read() {
        new_animation.push_back(event.clone());
    }

    animation_gate.event_queues.push(new_animation);
    animation_gate.timer.tick(time.delta());

    if animation_gate.timer.finished() {
        for event_queue in &mut animation_gate.event_queues {
            for _ in 0..PATHFINDING_TILE_BATCH {
                if let Some(event) = event_queue.pop_front() {
                    for (tile, mut anim_state) in &mut anim_states {
                        if tile.id == event.tile_id {
                            if anim_state.ran == false {
                                anim_state.initiated = true;
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
