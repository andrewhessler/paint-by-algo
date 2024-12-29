use std::sync::atomic::{AtomicUsize, Ordering};

use bevy::prelude::*;

use crate::{
    animation::tile::{TileAnimation, TileAnimationState},
    collision::collidable::Collidable,
    entities::ground::{GROUND_L_BORDER, GROUND_T_BORDER},
    wallbuilding::wall_manager::{WallAction, WallEvent},
};

use super::ground::{GROUND_H, GROUND_W};

pub const TEMP_TILE_COLOR_1: Color = Color::hsl(117., 0.67, 0.58);
pub const TEMP_TILE_COLOR_2: Color = Color::hsla(171., 0.35, 0.68, 0.50);
const END_TILE_COLOR: Color = Color::hsl(360., 0.80, 0.50);
pub const WALL_COLOR: Color = Color::hsl(0., 0.71, 0.19);

pub const TILE_SIZE: f32 = 50.;
pub const TILE_OFFSET: f32 = TILE_SIZE / 2.;
pub const ROW_COUNT: usize = (GROUND_H / TILE_SIZE) as usize;
pub const COL_COUNT: usize = (GROUND_W / TILE_SIZE) as usize;

pub mod emit_current;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Open,
    End,
    Wall,
}

#[derive(Component, Debug)]
pub struct Tile {
    pub id: usize,
    pub row: usize,
    pub col: usize,
    pub tile_type: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            id: get_tile_id(),
            row: 0, // TODO: turn this into an option after system breakout, maybe
            col: 0, // TODO: turn this into an option after system breakout, maybe
            tile_type: TileType::Open,
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tile_grid)
            .add_systems(FixedUpdate, handle_wall_event);
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
            let mut anim_enabled = TileAnimationState::Ran;
            let mut tile_type = TileType::Open;
            if r == ROW_COUNT - (ROW_COUNT / 2) && c == COL_COUNT - (COL_COUNT / 2) {
                // ending tile, maybe find way to extract this into a component? Want to make it
                // modifiable by user at runtime, should use an attribute for that, right?
                visibility = Visibility::Visible;
                tile_type = TileType::End;

                anim_enabled = TileAnimationState::Disabled;
                tile_color = END_TILE_COLOR;
            } else if r < 15 && r > 10 && c > 2 && c < 140 {
                visibility = Visibility::Visible;
                tile_type = TileType::Wall;
                anim_enabled = TileAnimationState::Ran;
                tile_color = WALL_COLOR;
            }

            let mut entity = commands.spawn((
                Tile {
                    row: r,
                    col: c,
                    tile_type: tile_type.clone(),
                    ..Default::default()
                },
                TileAnimation {
                    state: anim_enabled,
                    ..Default::default()
                },
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(tile_color)),
                Transform::from_xyz(x_position, y_position, 0.5),
                visibility,
            ));

            if tile_type == TileType::Wall {
                entity.insert(Collidable);
            }
        }
    }
}

fn handle_wall_event(
    mut commands: Commands,
    mut walls_reader: EventReader<WallEvent>,
    mut q_tiles: Query<(Entity, &mut Tile)>,
) {
    for event in walls_reader.read() {
        for (entity_id, mut tile) in &mut q_tiles {
            if event.action == WallAction::Added {
                if event.tile_id == tile.id {
                    tile.tile_type = TileType::Wall;
                    commands.entity(entity_id).insert(Collidable);
                }
            }
        }
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_tile_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}
