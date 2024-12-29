use bevy::prelude::*;

const GROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub(crate) const GROUND_W: f32 = 3400.; // 3400
pub(crate) const GROUND_H: f32 = 2000.; // 2000

pub(crate) const GROUND_L_BORDER: f32 = -GROUND_W / 2.;
pub(crate) const GROUND_T_BORDER: f32 = GROUND_H / 2.;
pub(crate) const GROUND_R_BORDER: f32 = GROUND_W / 2.;
pub(crate) const GROUND_B_BORDER: f32 = -GROUND_H / 2.;

#[derive(Component)]
struct Ground;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Ground,
        Mesh2d(meshes.add(Rectangle::new(GROUND_W, GROUND_H))),
        MeshMaterial2d(materials.add(GROUND_COLOR)),
        Transform::from_xyz(0., 0., 0.),
    ));
}
