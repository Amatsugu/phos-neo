use asset_loader::create_asset_loader;
use bevy::{
	asset::{Asset, AssetLoader, AsyncReadExt},
	ecs::system::Resource,
	reflect::TypePath,
};
use serde::{Deserialize, Serialize};
#[derive(Resource, Debug)]
pub struct TileManager {
	pub tiles: Vec<Handle<TileAsset>>,
}

impl Default for TileManager {
	fn default() -> Self {
		Self { tiles: vec![] }
	}
}

impl TileManager {
	pub fn register_tile(&mut self, tile: Handle<TileAsset>) -> usize {
		let id = self.tiles.len();
		self.tiles.push(tile);
		return id;
	}
}

#[derive(Serialize, Deserialize, Debug, TypePath, Asset)]
pub struct TileAsset {
	#[serde(skip)]
	pub id: usize,
	pub name: String,
	pub texture_id: u32,
	#[serde(skip)]
	pub texture: String,
	pub side_texture_id: u32,
	#[serde(skip)]
	pub side_texture: String,
}

create_asset_loader!(TileAssetPlugin, TileAssetLoader, TileAsset, &["tile.json"]);
