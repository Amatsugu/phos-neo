use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

use super::chunk::Chunk;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GenerationConfig {
	pub sea_level: f64,
	pub border_size: f32,
	pub biome_blend: usize,
	pub biome_dither: f64,
	pub moisture_noise: NoiseConfig,
	pub temperature_noise: NoiseConfig,
	pub continent_noise: NoiseConfig,
	pub size: UVec2,
}

impl GenerationConfig {
	pub fn get_total_width(&self) -> usize {
		return self.size.x as usize * Chunk::SIZE;
	}
	pub fn get_total_height(&self) -> usize {
		return self.size.y as usize * Chunk::SIZE;
	}
}

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
pub struct NoiseConfig {
	pub scale: f64,
	pub layers: Vec<GeneratorLayer>,
}

#[derive(Reflect, InspectorOptions, Serialize, Deserialize, Debug, Clone, Default)]
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
