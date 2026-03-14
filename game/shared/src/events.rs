use bevy::prelude::*;
use hex::prelude::*;

#[derive(Message)]
pub enum TileModifiedEvent
{
	HeightChanged(HexCoord, f32),
	TypeChanged(HexCoord, usize),
}

#[derive(Message)]
pub struct ChunkModifiedEvent
{
	pub index: usize,
}
