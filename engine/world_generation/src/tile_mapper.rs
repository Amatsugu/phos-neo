use asset_loader::create_asset_loader;
use bevy::prelude::*;
use bevy::{asset::Asset, reflect::TypePath};
use serde::{Deserialize, Serialize};

use crate::tile_manager::TileAsset;

pub struct TileMapper;

#[derive(Serialize, Deserialize, Debug, TypePath, Asset)]
pub struct TileMapperAsset {
	#[serde(skip)]
	pub tiles: Vec<Handle<TileAsset>>,
	pub tiles_path: Vec<String>,
	pub thresholds: Vec<f32>,
}

impl TileMapperAsset {
	pub fn sample_tile(&self, height: f32) -> AssetId<TileAsset> {
		for i in 0..self.thresholds.len() {
			let t = self.thresholds[i];
			if t >= height {
				return self.tiles[i].id();
			}
		}
		return self.tiles.last().unwrap().id();
	}
}

create_asset_loader!(
	TileMapperAssetPlugin,
	TileMapperAssetLoader,
	TileMapperAsset,
	&["mapper", "mapper.ron"],;
	tiles_path -> tiles
	?
);
