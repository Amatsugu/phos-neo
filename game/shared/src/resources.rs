use bevy::prelude::*;
use world_generation::hex_utils::HexCoord;

#[derive(Resource, Default)]
pub struct TileUnderCursor(pub Option<TileContact>);

#[derive(Clone, Copy)]
pub struct TileContact {
	pub tile: HexCoord,
	pub point: Vec3,
	pub surface: Vec3,
}

impl TileContact {
	pub fn new(tile: HexCoord, contact: Vec3, surface: Vec3) -> Self {
		return Self {
			tile,
			point: contact,
			surface,
		};
	}
}
