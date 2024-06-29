use asset_loader::create_asset_loader;
use bevy::prelude::{self, Resource};
use serde::{Deserialize, Serialize};

use super::building_asset::BuildingAsset;

#[derive(Serialize, Deserialize, Debug, TypePath, Asset)]
pub struct BuildingDatabase {
	pub hq: u32,
	pub buildings_paths: Vec<String>,
	pub buildings: Vec<BuildingAsset>,
}

create_asset_loader!(
	BuildingDatabasePlugin,
	BuildingDatabaseLoader,
	BuildingDatabase,
	BuildingDatabaseState,
	&["building.db.json"],;
	buildings_paths -> buildings
	?
);
