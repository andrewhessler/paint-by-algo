use std::{collections::VecDeque, sync::atomic::AtomicUsize, time::Duration};

use bevy::prelude::*;

use crate::{
    entities::tile::Tile,
    pathfinding::emit_pathfinding::{PathEvent, PathfindingEvent, PathfindingNode},
};

use super::{TileAnimation, TileAnimationState};

const PATHFINDING_ANIMATION_DELAY_MS: u64 = 1;
const PATHFINDING_TILE_BATCH: u64 = 8;

pub struct PathfindingTileAnimationPlugin;

impl Plugin for PathfindingTileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingAnimationGate {
            timer: Timer::new(
                Duration::from_millis(PATHFINDING_ANIMATION_DELAY_MS),
                TimerMode::Repeating,
            ),
            event_queues: Vec::new(),
        })
        .add_systems(Update, (initiate_animation, handle_pathfinding_event));
    }
}

#[derive(Resource)]
struct PathfindingAnimationGate {
    timer: Timer,
    event_queues: Vec<VecDeque<AnimationFromPathfinding>>,
}

struct AnimationFromPathfinding {
    event: PathfindingNode,
    color: usize,
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_calc_number() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

fn initiate_animation(
    mut anim_states: Query<(&Tile, &mut TileAnimation)>,
    mut animation_gate: ResMut<PathfindingAnimationGate>,
    time: Res<Time>,
) {
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

fn handle_pathfinding_event(
    mut pathfinding_event_reader: EventReader<PathfindingEvent>,
    mut path_event_reader: EventReader<PathEvent>,
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
}
