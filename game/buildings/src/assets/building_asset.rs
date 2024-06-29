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
	pub prefab: (),

	pub cost: Vec<ResourceIdentifier>,
	pub consumption: Vec<ResourceIdentifier>,
	pub production: Vec<ResourceIdentifier>,
}
