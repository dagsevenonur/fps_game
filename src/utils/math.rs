use bevy::prelude::*;

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

pub fn lerp_vec3(start: Vec3, end: Vec3, t: f32) -> Vec3 {
    Vec3::new(
        lerp(start.x, end.x, t),
        lerp(start.y, end.y, t),
        lerp(start.z, end.z, t),
    )
}

pub fn smooth_damp(
    current: f32,
    target: f32,
    current_velocity: &mut f32,
    smooth_time: f32,
    delta_time: f32,
) -> f32 {
    let smooth_time = smooth_time.max(0.0001);
    let omega = 2.0 / smooth_time;

    let x = omega * delta_time;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    
    let change = current - target;
    let temp = (*current_velocity + omega * change) * delta_time;
    
    *current_velocity = (*current_velocity - omega * temp) * exp;
    
    target + (change + temp) * exp
} 