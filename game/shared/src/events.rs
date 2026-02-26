use bevy::prelude::*;
use world_generation::hex_utils::*;

#[derive(Message)]
pub enum TileModifiedEvent {
	HeightChanged(HexCoord, f32),
	TypeChanged(HexCoord, usize),
}

#[derive(Message)]
pub struct ChunkModifiedEvent {
	pub index: usize,
}
