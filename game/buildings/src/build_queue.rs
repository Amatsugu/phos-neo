
use bevy::prelude::Resource;
use shared::building::BuildingIdentifier;
use world_generation::hex_utils::HexCoord;

#[derive(Resource)]
pub struct BuildQueue {
	pub queue: Vec<QueueEntry>,
}

impl Default for BuildQueue {
	fn default() -> Self {
		Self {
			queue: Default::default(),
		}
	}
}

#[derive(PartialEq, Eq)]
pub struct QueueEntry {
	pub building: BuildingIdentifier,
	pub pos: HexCoord,
}
