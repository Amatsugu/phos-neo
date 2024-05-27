use asset_loader::create_asset_loader;
use bevy::{
	asset::{Asset, Handle},
	reflect::TypePath,
};
use serde::{Deserialize, Serialize};

use crate::tile_mapper::TileMapperAsset;

#[derive(Serialize, Deserialize, Debug, TypePath, Asset, Clone)]
pub struct BiomePainterAsset {
	#[serde(skip)]
	pub biomes: Vec<Handle<TileMapperAsset>>,
	pub biomes_path: [String; 16],
}

impl BiomePainterAsset {
	pub fn sample_biome(&self, moisture: f32, temperature: f32) -> Handle<TileMapperAsset> {
		let x = (moisture.clamp(0., 1.) * 3.).ceil() as usize;
		let y = (temperature.clamp(0., 1.) * 3.).ceil() as usize;
		return self.biomes[x + y * 4].clone();
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
