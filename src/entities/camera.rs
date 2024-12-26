use bevy::prelude::*;

pub struct SceneCameraPlugin;

impl Plugin for SceneCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    println!("Current projection scale {}", projection.scale);
    projection.scale = 2.0;
    commands.spawn((Camera2d, projection));
}
