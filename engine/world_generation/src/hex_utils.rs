use bevy::prelude::*;

pub const OUTER_RADIUS: f32 = 1.;
pub const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866025404;

pub fn to_hex_pos(pos: Vec3) -> Vec3 {
	let x = (pos.x + pos.z * 0.5 - (pos.z / 2.).floor()) * (INNER_RADIUS * 2.);
	return Vec3::new(x, pos.y, pos.z * OUTER_RADIUS * 1.5);
}
