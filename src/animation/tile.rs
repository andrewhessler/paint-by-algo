use bevy::prelude::*;

use pathfinding::PathfindingTileAnimationPlugin;
use terrain::TerrainTileAnimationPlugin;

pub mod pathfinding;
pub mod terrain;

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;

pub struct TileAnimationPlugin;

impl Plugin for TileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerrainTileAnimationPlugin, PathfindingTileAnimationPlugin))
            .add_systems(Update, animate_tile);
    }
}

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
