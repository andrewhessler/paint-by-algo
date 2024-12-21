use bevy::prelude::*;
use entities::camera::SceneCameraPlugin;
use entities::ground::GroundPlugin;
use entities::player::PlayerPlugin;
use entities::tile::TilePlugin;
use systems::player_movement::PlayerMovementPlugin;

mod debug;
mod entities;
mod systems;

const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;

#[derive(Component)]
struct AnimateTile {
    enabled: bool,
    growing: bool,
    shrinking: bool,
    initiated: bool,
    ran: bool,
}

#[derive(Component)]
struct MyMovementState {
    position: Vec3,
    rotation: Quat,
    velocity: Vec3,
}

#[derive(Component)]
struct OldMovementState {
    position: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, GroundPlugin, TilePlugin, SceneCameraPlugin))
        .add_plugins(PlayerMovementPlugin)
        // .add_systems(
        //     FixedUpdate,
        //     (
        //         player_movement,
        //         animate_tile,
        //         set_current_tile,
        //         draw_path_to_end,
        //     ),
        // )
        // .add_systems(Update, transform_movement_interpolate)
        .run();
}

// fn animate_tile(
//     time: Res<Time>,
//     mut tiles: Query<(&mut Transform, &mut Tile, &mut AnimateTile, &mut Visibility)>,
// ) {
//     for (mut xf, _tile, mut animate_state, mut vis) in &mut tiles {
//         if animate_state.initiated && !animate_state.ran && animate_state.enabled {
//             if *vis == Visibility::Hidden {
//                 vis.toggle_visible_hidden();
//             }
//             if !animate_state.shrinking {
//                 animate_state.growing = true;
//             }
//
//             if animate_state.growing {
//                 xf.scale += TILE_ANIMATION_STEP * time.delta_secs();
//             }
//
//             if animate_state.shrinking {
//                 xf.scale -= TILE_ANIMATION_STEP * time.delta_secs();
//             }
//
//             if xf.scale.y > TILE_ANIMATION_MAX_SCALE {
//                 animate_state.growing = false;
//                 animate_state.shrinking = true;
//             }
//
//             if xf.scale.y < 1. {
//                 animate_state.shrinking = false;
//                 animate_state.ran = true;
//                 xf.scale = Vec3::new(1., 1., 1.);
//                 if *vis == Visibility::Visible {
//                     vis.toggle_visible_hidden();
//                 }
//             }
//         }
//     }
// }
//
// fn set_current_tile(
//     player: Query<(&Transform, &Player)>,
//     mut tiles: Query<(&Transform, &mut Tile, &mut AnimateTile)>,
// ) {
//     let player_position = player.single().0.translation;
//     for (xf, mut tile_props, mut tile_anim) in &mut tiles {
//         let tile_position = &xf.translation;
//         let is_in_x = player_position.x < tile_position.x + (TILE_SIZE / 2.)
//             && player_position.x > tile_position.x - (TILE_SIZE / 2.);
//
//         let is_in_y = player_position.y < tile_position.y + (TILE_SIZE / 2.)
//             && player_position.y > tile_position.y - (TILE_SIZE / 2.);
//
//         if is_in_x && is_in_y {
//             tile_props.current = true;
//             tile_anim.initiated = true;
//         } else {
//             tile_props.current = false;
//             if tile_anim.ran == true {
//                 tile_anim.initiated = false;
//                 tile_anim.ran = false;
//             }
//         }
//     }
// }
//
//
// fn draw_path_to_end(time: Res<Time>, mut tiles: Query<(&mut Tile)>, mut count: Local<u32>) {
//     *count += 1;
//     if *count == 1 {
//         let tiles: Vec<&Tile> = tiles.iter().collect();
//         for tile in tiles {
//             println!("{:?}", tile);
//         }
//     }
// }
