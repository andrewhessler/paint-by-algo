use bevy::prelude::*;

use crate::level_instantiation::ground::{GROUND_L_BORDER, GROUND_T_BORDER};

use super::ground::{GROUND_H, GROUND_W};

const TEMP_TILE_COLOR_1: Color = Color::hsl(316., 0.31, 0.58);
const TEMP_TILE_COLOR_2: Color = Color::hsl(225., 0.31, 0.38);
const END_TILE_COLOR: Color = Color::hsl(360., 0.80, 0.50);

const TILE_SIZE: f32 = 50.;
const TILE_OFFSET: f32 = TILE_SIZE / 2.;
const ROW_COUNT: i32 = (GROUND_H / TILE_SIZE) as i32;
const COL_COUNT: i32 = (GROUND_W / TILE_SIZE) as i32;

#[derive(Component, Debug)]
struct Tile {
    row: i32,
    col: i32,
    current: bool,
    is_end: bool,
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
            if r == ROW_COUNT - 1 && c == COL_COUNT - 1 {
                visibility = Visibility::Visible;
                is_end = true;
                anim_enabled = false;
                tile_color = END_TILE_COLOR;
            }

            commands.spawn((
                Tile {
                    row: r,
                    col: c,
                    current: false,
                    is_end,
                },
                // AnimateTile {
                //     enabled: anim_enabled,
                //     growing: false,
                //     shrinking: false,
                //     initiated: false,
                //     ran: false,
                // },
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(tile_color)),
                Transform::from_xyz(x_position, y_position, 0.5),
                visibility,
            ));
        }
    }
}
