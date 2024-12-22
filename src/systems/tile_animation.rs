use bevy::prelude::*;

use crate::entities::tile::Tile;

use super::emit_current_tile::CurrentTileEvent;

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;

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
        app.add_systems(
            FixedUpdate,
            (animate_tile, initiate_animation_by_activated_tile),
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

fn initiate_animation_by_activated_tile(
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
