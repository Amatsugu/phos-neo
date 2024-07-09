use asset_loader::create_asset_loader;
use bevy::{asset::Asset, reflect::TypePath, render::render_resource::encase::rts_array::Length};
use serde::{Deserialize, Serialize};

use crate::{biome_asset::BiomeAsset, map::biome_map::BiomeData};

#[derive(Serialize, Deserialize, Debug, TypePath, Asset, Clone)]
pub struct BiomePainterAsset {
	#[serde(skip)]
	pub biomes: Vec<Handle<BiomeAsset>>,
	pub biomes_path: Vec<String>,
}

impl BiomePainterAsset {
	pub fn sample_biome(&self, assets: &Assets<BiomeAsset>, data: &BiomeData) -> AssetId<BiomeAsset> {
		assert!(self.biomes.length() != 0, "There are no biomes");
		let mut biome = self.biomes.first().unwrap().id();
		let mut dist = f32::INFINITY;

		for b in &self.biomes {
			let asset = assets.get(b.id()).unwrap();
			let d = asset.distance(data.into());
			if d < dist {
				biome = b.id();
				dist = d;
			}
		}

		return biome;
	}

	pub fn build(&self, assets: &Assets<BiomeAsset>) -> BiomePainter {
		let mut biomes = Vec::with_capacity(self.biomes.len());
		for b in &self.biomes {
			let asset = assets.get(b.id()).unwrap();
			biomes.push(asset.clone());
		}
		return BiomePainter { biomes };
	}
}

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

create_asset_loader!(
	BiomePainterPlugin,
	BiomePainterLoader,
	BiomePainterAsset,
	BiomePainterLoadState,
	&["bimoes.json"],
	;
	biomes_path -> biomes
	?
);
