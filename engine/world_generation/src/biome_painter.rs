use asset_loader::create_asset_loader;
use bevy::prelude::*;
use bevy::{asset::Asset, reflect::TypePath, render::render_resource::encase::rts_array::Length};
use serde::{Deserialize, Serialize};

use crate::{biome_asset::BiomeAsset, map::biome_map::BiomeData};

#[derive(Resource)]
pub struct BiomePainter {
	pub biomes: Vec<BiomeAsset>,
}

impl BiomePainter {
	pub fn sample_biome(&self, data: &BiomeData) -> &BiomeAsset {
		assert!(self.biomes.length() != 0, "There are no biomes");
		let mut biome = &self.biomes[0];
		let mut dist = f32::INFINITY;

		for b in &self.biomes {
			let d = b.distance(data.into());
			if d < dist {
				biome = b;
				dist = d;
			}
		}

		return biome;
	}

	pub fn sample_biome_index(&self, data: &BiomeData) -> usize {
		assert!(self.biomes.length() != 0, "There are no biomes");
		let mut biome = 0;
		let mut dist = f32::INFINITY;

		for i in 0..self.biomes.len() {
			let d = self.biomes[i].distance(data.into());
			if d < dist {
				biome = i;
				dist = d;
			}
		}

		return biome;
	}
}

// create_asset_loader!(
// 	BiomePainterPlugin,
// 	BiomePainterLoader,
// 	BiomePainterAsset,
// 	BiomePainterLoadState,
// 	&["bimoes.json"],
// 	;
// 	biomes_path -> biomes
// 	?
// );
