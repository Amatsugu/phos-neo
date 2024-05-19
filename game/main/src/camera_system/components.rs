use bevy::prelude::*;
use world_generation::{hex_utils::SHORT_DIAGONAL, prelude::Chunk};

#[derive(Component, Reflect)]
#[reflect(Component)]
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
			max_height: 120.,
			speed: 30.,
			zoom_speed: 0.3,
			min_angle: (20. as f32).to_radians(),
			max_angle: 1.,
		}
	}
}

#[derive(Component)]
pub struct PhosCameraTargets {
	pub height: f32,
	pub forward: Vec3,
	pub last_height: f32,
	pub anim_time: f32,
	pub rotate_time: f32,
}

impl Default for PhosCameraTargets {
	fn default() -> Self {
		Self {
			height: Default::default(),
			forward: Vec3::Z,
			last_height: Default::default(),
			anim_time: Default::default(),
			rotate_time: Default::default(),
		}
	}
}

#[derive(Component, Default)]
pub struct CameraBounds {
	pub min: Vec2,
	pub max: Vec2,
}

impl CameraBounds {
	pub fn from_size(size: UVec2) -> Self {
		let padding = Chunk::WORLD_SIZE;
		return Self {
			min: Vec2::ZERO - padding,
			max: Vec2::new(
				(size.x as usize * Chunk::SIZE) as f32 * SHORT_DIAGONAL,
				(size.y * Chunk::SIZE as u32) as f32 * 1.5,
			) + padding,
		};
	}
}
