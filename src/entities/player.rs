use bevy::prelude::*;
use movement::PlayerMovement;

pub mod movement;

const PLAYER_COLOR: Color = Color::hsl(0., 1.0, 0.5);
const PLAYER_SPEED: f32 = 300.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        PlayerMovement::from_velocity_and_up_direction((PLAYER_SPEED, PLAYER_SPEED), (-1., 0.)),
        Mesh2d(meshes.add(CircularSector::new(15., 0.7))),
        MeshMaterial2d(materials.add(PLAYER_COLOR)),
        Transform::from_xyz(10.85, 10.10, 1.),
    ));
}
