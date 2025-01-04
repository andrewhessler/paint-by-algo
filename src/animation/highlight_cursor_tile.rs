use bevy::prelude::*;

use crate::entities::tile::{emit_current::CurrentMouseTileEvent, TILE_SIZE};

pub struct HighlightCursorTilePlugin;

impl Plugin for HighlightCursorTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_highlighter)
            .add_systems(Update, move_and_show_highlighter);
    }
}

#[derive(Component)]
pub struct Highlighter;

fn spawn_highlighter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let visibility = Visibility::Hidden;
    commands.spawn((
        Highlighter,
        Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.5),
        visibility,
    ));
}

fn move_and_show_highlighter(
    mut highlighter: Query<(&mut Transform, &mut Visibility), With<Highlighter>>,
    mut current_mouse_tile_reader: EventReader<CurrentMouseTileEvent>,
) {
    for event in current_mouse_tile_reader.read() {
        let (mut hl_xf, mut vis) = highlighter.single_mut();

        if let Some(_id) = event.id {
            hl_xf.translation = Vec3::new(event.world_x, event.world_y, 100.);
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }
}
