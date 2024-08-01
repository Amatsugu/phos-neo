use asset_loader::create_asset_loader;
use serde::{Deserialize, Serialize};

use crate::{prelude::NoiseConfig, tile_mapper::TileMapperAsset};

#[derive(Serialize, Deserialize, Asset, TypePath, Debug, Clone)]
pub struct BiomeAsset {
	pub moisture: f32,
	pub temperature: f32,
	pub continentality: f32,
	pub name: String,
	#[serde(skip)]
	pub tile_mapper: Handle<TileMapperAsset>,
	pub tile_mapper_path: String,
	pub noise: NoiseConfig,
}

impl BiomeAsset {
	pub fn distance(&self, data: Vec3) -> f32 {
		let a = Vec3::new(self.moisture, self.temperature, self.continentality);
		return (a - data).length();
	}
}

create_asset_loader!(
	BiomeAssetPlugin,
	BiomeAssetLoader,
	BiomeAsset,
	&["biome", "biome.ron"],
	tile_mapper_path -> tile_mapper
	;
	?
);
