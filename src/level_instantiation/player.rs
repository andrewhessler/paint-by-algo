use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::hsl(0., 1.0, 0.5);

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(CircularSector::new(15., 0.7))),
        MeshMaterial2d(materials.add(PLAYER_COLOR)),
        Transform::from_xyz(0., 0., 1.),
    ));
}
// commands.spawn((
//     Player,
//     MyMovementState {
//         position: Vec3::new(3., 4., 5.),
//         rotation: Quat::from_rotation_z((180.0_f32).to_radians()),
//         velocity: Vec3::new(300., 300., 0.),
//     },
//     OldMovementState {
//         position: Vec3::new(0., 0., 5.),
//     },
//     Mesh2d(meshes.add(CircularSector::new(15., 0.7))),
//     MeshMaterial2d(materials.add(PLAYER_COLOR)),
//     Transform::from_xyz(0., 0., 1.),
// ));
