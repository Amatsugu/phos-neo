use asset_loader::create_asset_loader;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::identifiers::ResourceIdentifier;

use crate::{
	buildings::{
		basic_building::BasicBuildingInfo, conduit_building::ResourceConduitInfo,
		factory_building::FactoryBuildingInfo, resource_gathering::ResourceGatheringBuildingInfo,
	},
	footprint::BuildingFootprint,
};

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

#[derive(Serialize, Deserialize, Debug, TypePath)]
pub enum BuildingType {
	Basic,
	Gathering(ResourceGatheringBuildingInfo),
	FactoryBuildingInfo(FactoryBuildingInfo),
	ResourceConduit(ResourceConduitInfo),
}

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub enum AnimationComponent {
	Rotation,
	Slider,
}
create_asset_loader!(
	BuildingAssetPlugin,
	BuildingAssetLoader,
	BuildingAsset,
	&["building", "building.ron"],
	prefab_path -> prefab
	;?
);
