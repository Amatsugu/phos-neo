use bevy::prelude::Resource;
use hex::prelude::*;
use shared::building::BuildingIdentifier;

#[derive(Resource, Default)]
pub struct BuildQueue
{
	pub queue: Vec<QueueEntry>,
}

#[derive(PartialEq, Eq)]
pub struct QueueEntry
{
	pub building: BuildingIdentifier,
	pub pos: HexCoord,
}
