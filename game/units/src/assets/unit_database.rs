use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use super::unit_asset::UnitAsset;

#[derive(Resource, AssetCollection)]
pub struct UnitDatabase {
	#[asset(key = "units", collection(typed))]
	pub units: Vec<Handle<UnitAsset>>,
}
