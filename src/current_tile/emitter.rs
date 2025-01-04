use bevy::{prelude::*, window::PrimaryWindow};

use crate::entities::{
    player::Player,
    tile::{Tile, TILE_SIZE},
};

pub struct EmitCurrentTilePlugin;

impl Plugin for EmitCurrentTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CurrentTileEvent>()
            .add_event::<CurrentMouseTileEvent>()
            .add_systems(FixedUpdate, (emit_current_tile, emit_current_mouse_tile));
    }
}

#[derive(Event)]
pub(crate) struct CurrentTileEvent {
    pub id: usize,
}

#[derive(Event)]
pub(crate) struct CurrentMouseTileEvent {
    pub id: Option<usize>,
    pub world_x: f32,
    pub world_y: f32,
}

fn emit_current_tile(
    player: Query<(&Transform, &Player)>,
    tiles: Query<(&Transform, &Tile)>,
    mut current_tile_writer: EventWriter<CurrentTileEvent>,
    mut prev_current_id: Local<Option<usize>>,
) {
    let player_position = player.single().0.translation;
    for (&xf, tile) in &tiles {
        let tile_position = &xf.translation;
        let is_in_x = player_position.x <= tile_position.x + (TILE_SIZE / 2.)
            && player_position.x >= tile_position.x - (TILE_SIZE / 2.);

        let is_in_y = player_position.y <= tile_position.y + (TILE_SIZE / 2.)
            && player_position.y >= tile_position.y - (TILE_SIZE / 2.);

        if is_in_x && is_in_y {
            if prev_current_id.map_or(true, |id| id != tile.id) {
                *prev_current_id = Some(tile.id);
                current_tile_writer.send(CurrentTileEvent { id: tile.id });
            }
        }
    }
}

fn emit_current_mouse_tile(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<(&Transform, &Tile)>,
    mut current_tile_writer: EventWriter<CurrentMouseTileEvent>,
    mut prev_current_id: Local<Option<usize>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        let mut found = false;
        for (&xf, tile) in &tiles {
            let (camera, camera_xf) = q_camera.single();
            if let Ok(world_coords) = camera
                .viewport_to_world(camera_xf, position)
                .map(|ray| ray.origin.truncate())
            {
                let tile_position = &xf.translation;
                let is_in_x = world_coords.x <= tile_position.x + (TILE_SIZE / 2.)
                    && world_coords.x >= tile_position.x - (TILE_SIZE / 2.);

                let is_in_y = world_coords.y <= tile_position.y + (TILE_SIZE / 2.)
                    && world_coords.y >= tile_position.y - (TILE_SIZE / 2.);

                if is_in_x && is_in_y {
                    found = true;
                    if prev_current_id.map_or(true, |id| id != tile.id) {
                        *prev_current_id = Some(tile.id);
                        current_tile_writer.send(CurrentMouseTileEvent {
                            id: Some(tile.id),
                            world_x: tile_position.x,
                            world_y: tile_position.y,
                        });
                    }
                }
            }
        }
        if !found {
            *prev_current_id = None;
            current_tile_writer.send(CurrentMouseTileEvent {
                id: None,
                world_x: 0.,
                world_y: 0.,
            });
        }
    } else {
        *prev_current_id = None;
        current_tile_writer.send(CurrentMouseTileEvent {
            id: None,
            world_x: 0.,
            world_y: 0.,
        });
    }
}
