use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct PlayerMovement {
    curr: TransformState,
    prev: TransformState,
    velocity: Vec2,
    up_dir: Vec2,
}

impl PlayerMovement {
    pub fn from_velocity_and_up_direction(
        velocity: (f32, f32),
        up_direction: (f32, f32),
    ) -> PlayerMovement {
        PlayerMovement {
            curr: TransformState::default(),
            prev: TransformState::default(),
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
        app.add_systems(FixedUpdate, player_movement)
            .add_systems(Update, transform_movement_interpolate);
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut movement: Query<(&Transform, &mut PlayerMovement)>,
) {
    for (xf, mut m) in &mut movement {
        if m.curr.position.is_none() {
            m.curr.position = Some(xf.translation);
        }
        let PlayerMovement {
            curr,
            prev,
            velocity,
            up_dir,
        } = &mut *m;
        *prev = curr.clone();
        let mut direction = Vec2::ZERO;

        if let Some(curr_position) = &mut curr.position {
            if keys.pressed(KeyCode::KeyW) {
                direction.y = 1.0;
            }

            if keys.pressed(KeyCode::KeyS) {
                direction.y = -1.0;
            }

            if keys.pressed(KeyCode::KeyA) {
                direction.x = -1.0;
            }

            if keys.pressed(KeyCode::KeyD) {
                direction.x = 1.0;
            }

            if direction != Vec2::ZERO {
                direction = direction.normalize();
                curr_position.x += direction.x * velocity.x * time.delta_secs();
                curr_position.y += direction.y * velocity.y * time.delta_secs();
            }
        }

        if let Some(curr_rotation) = &mut curr.rotation {
            let angle = up_dir.y.atan2(up_dir.x) - direction.x.atan2(direction.y);
            if direction != Vec2::ZERO {
                *curr_rotation = Quat::from_rotation_z(angle);
            };
        } else {
            curr.rotation = Some(xf.rotation);
        }
    }
}

fn transform_movement_interpolate(
    fixed_time: Res<Time<Fixed>>,
    mut movement: Query<(&mut Transform, &PlayerMovement)>,
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
