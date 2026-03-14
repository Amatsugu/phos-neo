use bevy::prelude::Resource;
use hex::prelude::*;
use shared::building::BuildingIdentifier;

#[derive(Resource)]
pub struct BuildQueue
{
	pub queue: Vec<QueueEntry>,
}

impl Default for BuildQueue
{
	fn default() -> Self
	{
		Self {
			queue: Default::default(),
		}
	}
}

#[derive(PartialEq, Eq)]
pub struct QueueEntry
{
	pub building: BuildingIdentifier,
	pub pos: HexCoord,
}
