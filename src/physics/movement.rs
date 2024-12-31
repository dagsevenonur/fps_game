use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

#[derive(Component)]
pub struct Gravity {
    pub force: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            force: 9.81,
        }
    }
}

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.linear * time.delta_seconds();
        transform.rotate_x(velocity.angular.x * time.delta_seconds());
        transform.rotate_y(velocity.angular.y * time.delta_seconds());
        transform.rotate_z(velocity.angular.z * time.delta_seconds());
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Gravity)>,
) {
    for (mut velocity, gravity) in query.iter_mut() {
        velocity.linear.y -= gravity.force * time.delta_seconds();
    }
} 