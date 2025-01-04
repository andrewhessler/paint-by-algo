use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(LogDiagnosticsPlugin::default());
    }
}

// fn print_entities(world: &World, query: Query<Entity>) {
//     for entity in &query {
//         let components = world.inspect_entity(entity);
//         for component in components {
//             println!("{:?}", component);
//         }
//     }
// }
