use bevy::prelude::*;

use crate::{
    collision::collidable::CollidedEvent,
    entities::ground::{
        GROUND_B_BORDER, GROUND_H, GROUND_L_BORDER, GROUND_R_BORDER, GROUND_T_BORDER, GROUND_W,
    },
};

use super::input::{InputAction, PlayerInput};

#[derive(Component)]
pub(crate) struct PlayerMovement {
    curr: TransformState,
    prev: TransformState,
    direction: Direction,
    velocity: Vec2,
    up_dir: Vec2,
}

#[derive(Default)]
struct Direction {
    vector: Vec2,
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

impl PlayerMovement {
    pub fn from_velocity_and_up_direction(
        velocity: (f32, f32),
        up_direction: (f32, f32),
    ) -> PlayerMovement {
        PlayerMovement {
            curr: TransformState::default(),
            prev: TransformState::default(),
            direction: Direction::default(),
            velocity: Vec2::new(velocity.0, velocity.1),
            up_dir: Vec2::new(up_direction.0, up_direction.1),
        }
    }
}

#[derive(Clone)]
struct TransformState {
    position: Option<Vec3>,
    rotation: Option<Quat>,
}

impl Default for TransformState {
    fn default() -> Self {
        TransformState {
            position: None,
            rotation: None,
        }
    }
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                player_movement,
                rebound_player,
                set_player_direction_from_input,
                teleport_player_at_bounds,
            ),
        )
        .add_systems(Update, (transform_movement_interpolate,));
    }
}

fn set_player_direction_from_input(
    mut player_input_event_reader: EventReader<PlayerInput>,
    mut movement: Query<&mut PlayerMovement>,
) {
    for event in player_input_event_reader.read() {
        for mut m in &mut movement {
            match event.action {
                InputAction::Pressed => match event.key {
                    KeyCode::KeyW => {
                        m.direction.up = true;
                        m.direction.vector.y = 1.0;
                    }
                    KeyCode::KeyS => {
                        m.direction.down = true;
                        m.direction.vector.y = -1.0;
                    }
                    KeyCode::KeyA => {
                        m.direction.left = true;
                        m.direction.vector.x = -1.0;
                    }
                    KeyCode::KeyD => {
                        m.direction.right = true;
                        m.direction.vector.x = 1.0;
                    }
                    _ => (),
                },
                InputAction::Released => match event.key {
                    KeyCode::KeyW => {
                        m.direction.up = false;
                        m.direction.vector.y = -1.0;
                    }
                    KeyCode::KeyS => {
                        m.direction.down = false;
                        m.direction.vector.y = 1.0;
                    }
                    KeyCode::KeyA => {
                        m.direction.left = false;
                        m.direction.vector.x = 1.0;
                    }
                    KeyCode::KeyD => {
                        m.direction.right = false;
                        m.direction.vector.x = -1.0;
                    }
                    _ => (),
                },
            };
            m.direction.vector.y = if !m.direction.up && !m.direction.down {
                0.0
            } else {
                m.direction.vector.y
            };

            m.direction.vector.x = if !m.direction.left && !m.direction.right {
                0.0
            } else {
                m.direction.vector.x
            };

            let _ = m.direction.vector.normalize();
        }
    }
}

fn rebound_player(
    mut collided_event_reader: EventReader<CollidedEvent>,
    mut movement: Query<&mut PlayerMovement>,
) {
    for event in collided_event_reader.read() {
        for mut p_mv in &mut movement {
            if let Some(curr_position) = &mut p_mv.curr.position {
                curr_position.y += 50. * event.rebound_direction.y;
                curr_position.x += 50. * event.rebound_direction.x;
            }
        }
    }
}

fn teleport_player_at_bounds(mut movement: Query<(&mut Transform, &mut PlayerMovement)>) {
    for (mut xf, mut state) in &mut movement {
        let mut teleported = false;
        if xf.translation.x > GROUND_R_BORDER {
            xf.translation.x -= GROUND_W;
            teleported = true;
        }
        if xf.translation.x < GROUND_L_BORDER {
            xf.translation.x += GROUND_W;
            teleported = true;
        }
        if xf.translation.y < GROUND_B_BORDER {
            xf.translation.y += GROUND_H;
            teleported = true;
        }
        if xf.translation.y > GROUND_T_BORDER {
            xf.translation.y -= GROUND_H;
            teleported = true;
        }

        if teleported {
            state.curr.position = Some(xf.translation);
            state.prev.position = Some(xf.translation);
        }
    }
}

fn player_movement(time: Res<Time>, mut movement: Query<(&Transform, &mut PlayerMovement)>) {
    for (xf, mut m) in &mut movement {
        if m.curr.position.is_none() {
            m.curr.position = Some(xf.translation);
        }
        let PlayerMovement {
            curr,
            prev,
            direction,
            velocity,
            up_dir,
        } = &mut *m;
        *prev = curr.clone();

        if let Some(curr_position) = &mut curr.position {
            if direction.vector != Vec2::ZERO {
                curr_position.x += direction.vector.x * velocity.x * time.delta_secs();
                curr_position.y += direction.vector.y * velocity.y * time.delta_secs();
            }
        }

        if let Some(curr_rotation) = &mut curr.rotation {
            let angle = up_dir.y.atan2(up_dir.x) - direction.vector.x.atan2(direction.vector.y);
            if direction.vector != Vec2::ZERO {
                *curr_rotation = Quat::from_rotation_z(angle);
            };
        } else {
            curr.rotation = Some(xf.rotation);
        }
    }
}

fn transform_movement_interpolate(
    fixed_time: Res<Time<Fixed>>,
    mut movement: Query<(&mut Transform, &mut PlayerMovement)>,
) {
    for (mut xf, state) in &mut movement {
        let a = fixed_time.overstep_fraction();

        if let (Some(prev_position), Some(curr_position)) =
            (state.prev.position, state.curr.position)
        {
            xf.translation = prev_position.lerp(curr_position, a);
        }

        if let Some(curr_rotation) = state.curr.rotation {
            xf.rotation = curr_rotation;
        }
    }
}
