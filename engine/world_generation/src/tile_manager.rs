use std::{collections::HashMap, fs};

use asset_loader::create_asset_loader;
use bevy::{
	asset::{Asset, AssetLoader, AsyncReadExt},
	ecs::system::Resource,
	reflect::TypePath,
};
use serde::{Deserialize, Serialize};
#[derive(Resource, Debug)]
pub struct TileManager {
	pub tiles: HashMap<u32, TileAsset>,
}

#[derive(Serialize, Deserialize, Debug, TypePath, Asset)]
pub struct TileAsset {
	#[serde(skip_serializing, skip_deserializing)]
	pub id: u32,
	pub name: String,
	pub texture_id: u32,
	pub side_texture_id: u32,
}

create_asset_loader!(TileAssetPlugin, TileAssetLoader, TileAsset, &["tile.json"]);

impl TileManager {
	pub fn new(path: &str) -> Result<Self, String> {
		let mut cur_id: u32 = 0;

		let paths = fs::read_dir(path);

		if paths.is_err() {
			return Err(paths.err().unwrap().to_string());
		}

		let mut manager = TileManager {
			tiles: HashMap::new(),
		};

		for dir_entry in paths.unwrap() {
			let cur_path = dir_entry.unwrap().path();
			if !cur_path.is_file() {
				continue;
			}
			let data = fs::read_to_string(cur_path);
			if data.is_err() {
				return Err(data.err().unwrap().to_string());
			}

			let result: Result<TileAsset, serde_json::Error> =
				serde_json::from_str(data.unwrap().as_str());

			if result.is_err() {
				return Err(result.err().unwrap().to_string());
			}

			let mut tile = result.unwrap();
			tile.id = cur_id;

			manager.tiles.insert(tile.id, tile);
			cur_id += 1;
		}

		return Ok(manager);
	}
}
