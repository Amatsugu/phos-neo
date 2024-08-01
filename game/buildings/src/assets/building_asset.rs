use asset_loader::create_asset_loader;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::resource::ResourceIdentifier;

use crate::footprint::BuildingFootprint;

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct BuildingAsset {
	pub name: String,
	pub description: String,
	pub footprint: BuildingFootprint,
	pub prefab_path: String,
	#[serde(skip)]
	pub prefab: Handle<Scene>,

	pub cost: Vec<ResourceIdentifier>,
	pub consumption: Vec<ResourceIdentifier>,
	pub production: Vec<ResourceIdentifier>,
}

create_asset_loader!(
	BuildingAssetPlugin,
	BuildingAssetLoader,
	BuildingAsset,
	&["building", "building.ron"],
	prefab_path -> prefab
	;?
);
