use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::hsl(0., 1.0, 0.5);
const GROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

const TEMP_TILE_COLOR_1: Color = Color::hsl(316., 0.31, 0.58);
const TEMP_TILE_COLOR_2: Color = Color::hsl(225., 0.31, 0.38);
const END_TILE_COLOR: Color = Color::hsl(360., 0.80, 0.50);

const FLOOR_W: f32 = 1200.;
const FLOOR_H: f32 = 600.;
const L_BORDER: f32 = -FLOOR_W / 2.;
const T_BORDER: f32 = FLOOR_H / 2.;
const TILE_SIZE: f32 = 50.;
const TILE_ANIMATION_MAX_SCALE: f32 = 1.3;
const TILE_ANIMATION_STEP: f32 = 3.0;
const TILE_OFFSET: f32 = TILE_SIZE / 2.;
const ROW_COUNT: i32 = (FLOOR_H / TILE_SIZE) as i32;
const COL_COUNT: i32 = (FLOOR_W / TILE_SIZE) as i32;

#[derive(Component)]
struct Ground;

#[derive(Component, Debug)]
struct Tile {
    row: i32,
    col: i32,
    current: bool,
    is_end: bool,
}

#[derive(Component)]
struct AnimateTile {
    enabled: bool,
    growing: bool,
    shrinking: bool,
    initiated: bool,
    ran: bool,
}

#[derive(Component)]
struct Player;

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
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                player_movement,
                animate_tile,
                set_current_tile,
                draw_path_to_end,
            ),
        )
        .add_systems(Update, transform_movement_interpolate)
        .run();
}

fn animate_tile(
    time: Res<Time>,
    mut tiles: Query<(&mut Transform, &mut Tile, &mut AnimateTile, &mut Visibility)>,
) {
    for (mut xf, _tile, mut animate_state, mut vis) in &mut tiles {
        if animate_state.initiated && !animate_state.ran && animate_state.enabled {
            if *vis == Visibility::Hidden {
                vis.toggle_visible_hidden();
            }
            if !animate_state.shrinking {
                animate_state.growing = true;
            }

            if animate_state.growing {
                xf.scale += TILE_ANIMATION_STEP * time.delta_secs();
            }

            if animate_state.shrinking {
                xf.scale -= TILE_ANIMATION_STEP * time.delta_secs();
            }

            if xf.scale.y > TILE_ANIMATION_MAX_SCALE {
                animate_state.growing = false;
                animate_state.shrinking = true;
            }

            if xf.scale.y < 1. {
                animate_state.shrinking = false;
                animate_state.ran = true;
                xf.scale = Vec3::new(1., 1., 1.);
                if *vis == Visibility::Visible {
                    vis.toggle_visible_hidden();
                }
            }
        }
    }
}

fn set_current_tile(
    player: Query<(&Transform, &Player)>,
    mut tiles: Query<(&Transform, &mut Tile, &mut AnimateTile)>,
) {
    let player_position = player.single().0.translation;
    for (xf, mut tile_props, mut tile_anim) in &mut tiles {
        let tile_position = &xf.translation;
        let is_in_x = player_position.x < tile_position.x + (TILE_SIZE / 2.)
            && player_position.x > tile_position.x - (TILE_SIZE / 2.);

        let is_in_y = player_position.y < tile_position.y + (TILE_SIZE / 2.)
            && player_position.y > tile_position.y - (TILE_SIZE / 2.);

        if is_in_x && is_in_y {
            tile_props.current = true;
            tile_anim.initiated = true;
        } else {
            tile_props.current = false;
            if tile_anim.ran == true {
                tile_anim.initiated = false;
                tile_anim.ran = false;
            }
        }
    }
}

fn create_tile_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    println!("Printing {} rows, {} cols", ROW_COUNT, COL_COUNT);
    for r in 0..ROW_COUNT {
        for c in 0..COL_COUNT {
            let x_position = L_BORDER + ((TILE_SIZE * c as f32) + TILE_OFFSET);
            let y_position = T_BORDER - ((TILE_SIZE * r as f32) + TILE_OFFSET);
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
                AnimateTile {
                    enabled: anim_enabled,
                    growing: false,
                    shrinking: false,
                    initiated: false,
                    ran: false,
                },
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(tile_color)),
                Transform::from_xyz(x_position, y_position, 0.5),
                visibility,
            ));
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    create_tile_grid(&mut commands, &mut meshes, &mut materials);

    commands.spawn((
        Ground,
        Mesh2d(meshes.add(Rectangle::new(FLOOR_W, FLOOR_H))),
        MeshMaterial2d(materials.add(GROUND_COLOR)),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((
        Player,
        MyMovementState {
            position: Vec3::new(3., 4., 5.),
            rotation: Quat::from_rotation_z((180.0_f32).to_radians()),
            velocity: Vec3::new(300., 300., 0.),
        },
        OldMovementState {
            position: Vec3::new(0., 0., 5.),
        },
        Mesh2d(meshes.add(CircularSector::new(15., 0.7))),
        MeshMaterial2d(materials.add(PLAYER_COLOR)),
        Transform::from_xyz(0., 0., 1.),
    ));

    commands.spawn(Camera2d);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut movement: Query<(&mut MyMovementState, &mut OldMovementState)>,
) {
    for (mut state, mut old_state) in &mut movement {
        let state = &mut *state;
        old_state.position = state.position;
        let mut y_dir = 0.0_f32;
        let mut x_dir = 0.0_f32;

        if keys.pressed(KeyCode::KeyW) {
            state.position.y += state.velocity.y * time.delta_secs();
            y_dir = -1.0;
        }

        if keys.pressed(KeyCode::KeyS) {
            state.position.y -= state.velocity.y * time.delta_secs();
            y_dir = 1.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            state.position.x -= state.velocity.x * time.delta_secs();
            x_dir = -1.0;
        }

        if keys.pressed(KeyCode::KeyD) {
            state.position.x += state.velocity.x * time.delta_secs();
            x_dir = 1.0;
        }

        let angle = x_dir.atan2(y_dir);
        state.rotation = if y_dir != 0.0 || x_dir != 0.0 {
            Quat::from_rotation_z(angle)
        } else {
            state.rotation
        };
    }
}

fn transform_movement_interpolate(
    fixed_time: Res<Time<Fixed>>,
    mut movement: Query<(&mut Transform, &mut MyMovementState, &mut OldMovementState)>,
) {
    for (mut xf, state, old_state) in &mut movement {
        let a = fixed_time.overstep_fraction();
        xf.translation = old_state.position.lerp(state.position, a);
        xf.rotation = state.rotation;
    }
}

fn draw_path_to_end(time: Res<Time>, mut tiles: Query<(&mut Tile)>, mut count: Local<u32>) {
    *count += 1;
    if *count == 1 {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        for tile in tiles {
            println!("{:?}", tile);
        }
    }
}
