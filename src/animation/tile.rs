use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use bevy::prelude::*;

use crate::{
    entities::tile::{emit_current::CurrentTileEvent, Tile, TileType, WALL_COLOR},
    pathfinding::emit_pathfinding::{PathfindingEvent, PathfindingEventType, PathfindingNode},
    wallbuilding::wall_manager::{WallAction, WallEvent},
};

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;
const PATHFINDING_ANIMATION_DELAY_MS: u64 = 1;
const PATHFINDING_TILE_BATCH: u64 = 5;

pub struct TileAnimationPlugin;

#[derive(Component, Default)]
pub struct TileAnimation {
    pub state: TileAnimationState,
    pub update_color: bool,
    pub color: usize,
    pub last_event: u32,
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
        app.add_systems(Startup, setup_pathfinding_animation_timer)
            .add_systems(
                FixedUpdate,
                (
                    animate_tile,
                    initiate_wall_bump_tile_animation,
                    handle_wall_event,
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
                    material.color = Color::hsl(color, 0.30, 0.73);
                }
            }
            if *vis == Visibility::Hidden {
                if let Some(material) = materials.get_mut(&mesh.0) {
                    material.color = Color::BLACK;
                }
                vis.toggle_visible_hidden();
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

fn initiate_wall_bump_tile_animation(
    mut anim_states: Query<(&Tile, &mut TileAnimation)>,
    mut tile_activated_reader: EventReader<CurrentTileEvent>,
) {
    for event in tile_activated_reader.read() {
        for (tile, mut anim) in &mut anim_states {
            if tile.tile_type == TileType::Wall {
                if tile.id == event.id {
                    if anim.state == TileAnimationState::Ran {
                        anim.state = TileAnimationState::Initiated;
                    }
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
    pub event: PathfindingNode,
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
        for node in &event.visited {
            new_animation.push_back(AnimationFromPathfinding {
                event: node.clone(),
                color,
            });
        }
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
                    for (tile, mut anim) in &mut anim_states {
                        if tile.id == event.event.tile_id {
                            anim.update_color = true;
                            anim.color = event.color;
                            anim.last_event = last_event;
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

fn handle_wall_event(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut walls_reader: EventReader<WallEvent>,
    mut q_tiles: Query<(
        &Tile,
        &mut TileAnimation,
        &MeshMaterial2d<ColorMaterial>,
        &mut Visibility,
    )>,
) {
    for event in walls_reader.read() {
        for (tile, mut anim, mesh, mut vis) in &mut q_tiles {
            if event.action == WallAction::Added {
                if event.tile_id == tile.id {
                    anim.state = TileAnimationState::Disabled;
                    *vis = Visibility::Visible;
                    if let Some(material) = materials.get_mut(&mesh.0) {
                        material.color = WALL_COLOR;
                    }
                }
            }
        }
    }
}
