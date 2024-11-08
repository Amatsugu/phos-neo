use bevy::{math::Direction3d, prelude::*};
use rayon::str;
use world_generation::{hex_utils::SHORT_DIAGONAL, prelude::Chunk};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PhosCamera {
	pub min_height: f32,
	pub max_height: f32,
	pub speed: f32,
	pub zoom_speed: f32,
	pub pan_speed: Vec2,
	pub min_angle: f32,
	pub max_angle: f32,
}

impl Default for PhosCamera {
	fn default() -> Self {
		Self {
			min_height: 10.,
			max_height: 420.,
			speed: 100.,
			pan_speed: Vec2::new(0.8, 0.5),
			zoom_speed: 200.,
			min_angle: (20. as f32).to_radians(),
			max_angle: 1.,
		}
	}
}
#[derive(Component, Reflect)]
pub struct PhosOrbitCamera {
	pub target: Vec3,
	pub distance: f32,
	pub forward: Vec3,
}

impl Default for PhosOrbitCamera {
	fn default() -> Self {
		Self {
			target: Default::default(),
			distance: 40.0,
			forward: Vec3::new(0.0, -0.5, 0.5).normalize(),
		}
	}
}

#[derive(Component, Default)]
pub struct CameraBounds {
	pub min: Vec2,
	pub max: Vec2,
}

impl CameraBounds {
	pub fn from_size(world_size: Vec2) -> Self {
		let padding = Chunk::WORLD_SIZE;
		return Self {
			min: Vec2::ZERO - padding,
			max: world_size + padding,
		};
	}
}
