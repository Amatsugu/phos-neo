use bevy::math::{IVec2, Vec3Swizzles};
use serde::{Deserialize, Serialize};
use world_generation::hex_utils::HexCoord;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingFootprint {
	pub footprint: Vec<IVec2>,
}

impl BuildingFootprint {
	pub fn get_footprint(&self, center: &HexCoord) -> Vec<HexCoord> {
		let c = center.hex.xy();
		return self.footprint.iter().map(|p| HexCoord::from_hex(*p + c)).collect();
	}

	pub fn get_footprint_rotated(&self, center: &HexCoord, rotation: i32) -> Vec<HexCoord> {
		let c = center.hex.xy();
		return self
			.footprint
			.iter()
			.map(|p| HexCoord::from_hex(*p + c).rotate_around(center, rotation))
			.collect();
	}

	pub fn get_neighbors(&self, center: &HexCoord) -> Vec<HexCoord> {
		todo!()
	}

	pub fn get_neighbors_rotated(&self, center: &HexCoord, rotation: i32) -> Vec<HexCoord> {
		todo!();
	}
}
