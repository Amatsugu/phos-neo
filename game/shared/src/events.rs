use bevy::prelude::*;
use world_generation::hex_utils::*;

#[derive(Event)]
pub enum TileModifiedEvent {
	HeightChanged(HexCoord, f32),
	TypeChanged(HexCoord, usize),
}

#[derive(Event)]
pub struct ChunkModifiedEvent {
	pub index: usize,
}
