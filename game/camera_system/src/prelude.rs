use bevy::prelude::*;

#[derive(Component)]
pub struct PhosCamera {
	pub min_height: f32,
	pub max_height: f32,
	pub speed: f32,
	pub zoom_speed: f32,
	pub min_angle: f32,
	pub max_angle: f32,
}

impl Default for PhosCamera {
	fn default() -> Self {
		Self {
			min_height: 10.,
			max_height: 100.,
			speed: 20.,
			zoom_speed: 20.,
			min_angle: 10.,
			max_angle: 80.,
		}
	}
}

#[derive(Component, Default)]
pub struct PhosCameraTargets {
	pub height: f32,
}

#[derive(Component, Default)]
pub struct CameraBounds {
	pub min: Vec2,
	pub max: Vec2,
}

impl CameraBounds {
	pub fn from_size(size: UVec2) -> Self {
		return Self {
			min: Vec2::ZERO,
			max: size.as_vec2(),
		};
	}
}
