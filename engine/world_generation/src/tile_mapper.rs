use asset_loader::create_asset_loader;
use bevy::prelude::*;
use bevy::{
	asset::{Asset, Handle},
	reflect::TypePath,
};
use serde::{Deserialize, Serialize};

use crate::tile_manager::TileAsset;

pub struct TileMapper;

#[derive(Serialize, Deserialize, Debug, TypePath, Asset)]
struct TileMapperAsset {
	#[serde(skip)]
	pub tiles: Vec<Handle<TileAsset>>,
	pub tiles_path: Vec<String>,
}

create_asset_loader!(
	TileMapperAssetPlugin,
	TileMapperAssetLoader,
	TileMapperAsset,
	&["mapper.json"],
);
