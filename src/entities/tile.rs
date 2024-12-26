use std::{
    sync::atomic::{AtomicUsize, Ordering},
    usize,
};

use bevy::prelude::*;

use crate::{
    entities::ground::{GROUND_L_BORDER, GROUND_T_BORDER},
    systems::tile_animation::TileAnimation,
};

use super::ground::{GROUND_H, GROUND_W};

pub const TEMP_TILE_COLOR_1: Color = Color::hsl(117., 0.67, 0.58);
pub const TEMP_TILE_COLOR_2: Color = Color::hsla(171., 0.35, 0.68, 0.50);
const END_TILE_COLOR: Color = Color::hsl(360., 0.80, 0.50);

pub const TILE_SIZE: f32 = 50.;
pub const TILE_OFFSET: f32 = TILE_SIZE / 2.;
pub const ROW_COUNT: usize = (GROUND_H / TILE_SIZE) as usize;
pub const COL_COUNT: usize = (GROUND_W / TILE_SIZE) as usize;

#[derive(Component, Debug)]
pub struct Tile {
    pub id: usize,
    pub row: usize,
    pub col: usize,
    pub is_end: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            id: get_tile_id(),
            row: 0, // TODO: turn this into an option after system breakout, maybe
            col: 0, // TODO: turn this into an option after system breakout, maybe
            is_end: false,
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tile_grid);
    }
}

fn spawn_tile_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Printing {} rows, {} cols", ROW_COUNT, COL_COUNT);
    for r in 0..ROW_COUNT {
        for c in 0..COL_COUNT {
            let x_position = GROUND_L_BORDER + ((TILE_SIZE * c as f32) + TILE_OFFSET);
            let y_position = GROUND_T_BORDER - ((TILE_SIZE * r as f32) + TILE_OFFSET);
            let mut tile_color = if (r + c) % 2 == 0 {
                TEMP_TILE_COLOR_1
            } else {
                TEMP_TILE_COLOR_2
            };

            let mut visibility = Visibility::Hidden;
            let mut is_end = false;
            let mut anim_enabled = true;
            if r == ROW_COUNT - (ROW_COUNT / 2) && c == COL_COUNT - (COL_COUNT / 2) {
                // ending tile, maybe find way to extract this into a component? Want to make it
                // modifiable by user at runtime, should use an attribute for that, right?
                visibility = Visibility::Visible;
                is_end = true;
                anim_enabled = false;
                tile_color = END_TILE_COLOR;
            }

            commands.spawn((
                Tile {
                    row: r,
                    col: c,
                    is_end,
                    ..Default::default()
                },
                TileAnimation {
                    enabled: anim_enabled,
                    ..Default::default()
                },
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(tile_color)),
                Transform::from_xyz(x_position, y_position, 0.5),
                visibility,
            ));
        }
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_tile_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}
