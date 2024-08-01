use bevy::{asset::Handle, prelude::Resource};

use super::building_asset::BuildingAsset;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Resource, AssetCollection)]
pub struct BuildingDatabase {
	#[asset(key = "buildings", collection(typed))]
	pub buildings: Vec<Handle<BuildingAsset>>,
}
