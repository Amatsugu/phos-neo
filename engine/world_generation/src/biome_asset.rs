use asset_loader::create_asset_loader;
use serde::{Deserialize, Serialize};

use crate::{prelude::GeneratorLayer, tile_mapper::TileMapperAsset};

#[derive(Serialize, Deserialize, Asset, TypePath, Debug, Clone)]
pub struct BiomeAsset {
	pub moisture: f32,
	pub temperature: f32,
	pub continentality: f32,
	pub name: String,
	#[serde(skip)]
	pub tile_mapper: Handle<TileMapperAsset>,
	pub tile_mapper_path: String,
	pub generator_layers: Vec<GeneratorLayer>,
}

impl BiomeAsset {
	pub fn distance(&self, moisture: f32, temperature: f32, continentality: f32) -> f32 {
		let a = Vec3::new(
			self.moisture - moisture,
			self.temperature - temperature,
			self.continentality - continentality,
		);
		return a.length();
	}
}

create_asset_loader!(
	BiomeAssetPlugin,
	BiomeAssetLoader,
	BiomeAsset,
	BiomeAssetLoadState,
	&["bimoe.json"],
	tile_mapper_path -> tile_mapper
	;
	?
);
