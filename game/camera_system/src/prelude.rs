use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PhosCamera {
	pub min_height: f32,
	pub max_height: f32,
	pub speed: f32,
	pub zoom_speed: f32,
}

pub struct CameraBounds {
	pub min: Vec2,
	pub max: Vec2,
}
