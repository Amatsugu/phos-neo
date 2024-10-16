use asset_loader::create_asset_loader;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use serde::{Deserialize, Serialize};
use shared::Tier;

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ResourceAsset {
	pub identifier: String,
	pub name: String,
	pub description: String,
	pub sprite_id: usize,
	pub tier: Tier,
}

create_asset_loader!(
	ResourceAssetPlugin,
	ResourceAssetLoader,
	ResourceAsset,
	&["res", "res.ron"],
	;?
);

#[derive(Resource, AssetCollection)]
pub struct ResourceDatabase {
	#[asset(key = "resources", collection(typed))]
	pub units: Vec<Handle<ResourceAsset>>,
}

impl ResourceDatabase {
	pub fn create_lookup(&self, assets: &Assets<ResourceAsset>) -> ResourceLookup {
		let mut identifiers = Vec::with_capacity(self.units.len());
		for handle in &self.units {
			if let Some(asset) = assets.get(handle.id()) {
				identifiers.push(asset.identifier.clone());
			}
		}
		return ResourceLookup {
			handles: self.units.clone(),
			identifiers,
		};
	}
}

#[derive(Resource)]
pub struct ResourceLookup {
	pub handles: Vec<Handle<ResourceAsset>>,
	pub identifiers: Vec<String>,
}
