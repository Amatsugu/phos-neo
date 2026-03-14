use bevy::math::{IVec2, Vec3Swizzles};
use serde::{Deserialize, Serialize};
use shared::coords::CoordsCollection;
use world_generation::hex_utils::HexCoord;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingFootprint
{
	pub footprint: Vec<IVec2>,
}

impl BuildingFootprint
{
	pub fn get_footprint(&self, position: &HexCoord) -> CoordsCollection
	{
		CoordsCollection::from_points(self.footprint.clone()).with_translation(position)
	}

	pub fn get_neighbors(&self, position: &HexCoord) -> CoordsCollection
	{
		let n_points: Vec<IVec2> = self
			.footprint
			.iter()
			.flat_map(|p| HexCoord::from_hex(*p).get_neighbors())
			.map(|c| c.hex.xy())
			.filter(|p| !self.footprint.contains(p))
			.collect();
		let mut out_points: Vec<IVec2> = Vec::with_capacity(n_points.len());
		for p in n_points
		{
			if out_points.contains(&p)
			{
				continue;
			}
			out_points.push(p);
		}
		return CoordsCollection::from_points(out_points).with_translation(position);
	}
}
