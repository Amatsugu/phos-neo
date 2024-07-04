use asset_loader::create_asset_loader;
use bevy::{
	asset::{Asset, Handle},
	reflect::TypePath,
	render::render_resource::encase::rts_array::Length,
};
use serde::{Deserialize, Serialize};

use crate::biome_asset::BiomeAsset;

#[derive(Serialize, Deserialize, Debug, TypePath, Asset, Clone)]
pub struct BiomePainterAsset {
	#[serde(skip)]
	pub biomes: Vec<Handle<BiomeAsset>>,
	pub biomes_path: Vec<String>,
}

impl BiomePainterAsset {
	pub fn sample_biome(
		&self,
		assets: &Assets<BiomeAsset>,
		moisture: f32,
		temperature: f32,
		continentality: f32,
	) -> Handle<BiomeAsset> {
		assert!(self.biomes.length() != 0, "There are no biomes");
		let mut biome = self.biomes.first().unwrap().clone();
		let mut dist = f32::INFINITY;

		for b in &self.biomes {
			let asset = assets.get(b).unwrap();
			let d = asset.distance(moisture, temperature, continentality);
			if d < dist {
				biome = b.clone();
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
