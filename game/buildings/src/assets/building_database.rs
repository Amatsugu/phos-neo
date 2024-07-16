use asset_loader::create_asset_loader;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::building_asset::BuildingAsset;

#[derive(Resource)]
pub struct BuildingDatabase{
	pub handle: Handle<BuildingDatabaseAsset>
}

#[derive(Serialize, Deserialize, Debug, TypePath, Asset)]
pub struct BuildingDatabaseAsset {
	pub hq: u32,
	pub buildings_paths: Vec<String>,
	#[serde(skip)]
	pub buildings: Vec<Handle<BuildingAsset>>,
}

create_asset_loader!(
	BuildingDatabasePlugin,
	BuildingDatabaseLoader,
	BuildingDatabaseAsset,
	BuildingDatabaseState,
	&["buildings.db.json"],;
	buildings_paths -> buildings
	?
);
