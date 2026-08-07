use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct TileVizTextures
{
	#[asset(key = "select")]
	pub select: Handle<Image>,
	#[asset(key = "select_emission")]
	pub select_emission: Handle<Image>,
	#[asset(key = "powered")]
	pub powered: Handle<Image>,
	#[asset(key = "star")]
	pub star: Handle<Image>,
	#[asset(key = "star_emission")]
	pub star_emission: Handle<Image>,
	#[asset(key = "x")]
	pub x: Handle<Image>,
}
