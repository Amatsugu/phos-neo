use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use super::chunk::Chunk;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GenerationConfig {
	pub noise_scale: f64,
	pub sea_level: f64,
	pub border_size: f32,
	pub size: UVec2,
	pub layers: Vec<GeneratorLayer>,
}

impl GenerationConfig {
	pub fn get_total_width(&self) -> usize {
		return self.size.x as usize * Chunk::SIZE;
	}
	pub fn get_total_height(&self) -> usize {
		return self.size.y as usize * Chunk::SIZE;
	}
}

#[derive(Reflect, InspectorOptions)]
pub struct GeneratorLayer {
	pub strength: f64,
	pub min_value: f64,
	pub base_roughness: f64,
	pub roughness: f64,
	pub persistence: f64,
	pub is_rigid: bool,
	pub weight: f64,
	pub weight_multi: f64,
	pub layers: usize,
	pub first_layer_mask: bool,
}
