use bevy::prelude::*;
use world_generation::hex_utils::HexCoord;

#[derive(Default, Debug, Reflect)]
pub struct CoordsCollection
{
	points: Vec<IVec2>,
	origin: IVec2,
	translation: IVec2,
	rotation: i32,
}

impl CoordsCollection
{
	pub fn from_hex(coords: Vec<HexCoord>) -> Self
	{
		CoordsCollection {
			points: coords.iter().map(|c| c.hex.xy()).collect(),
			..default()
		}
	}

	pub fn from_points(points: Vec<IVec2>) -> Self
	{
		CoordsCollection { points, ..default() }
	}

	pub fn with_translation(mut self, translation: &HexCoord) -> Self
	{
		self.translation = translation.hex.xy();
		return self;
	}

	pub fn with_translation_vec(mut self, translation: IVec2) -> Self
	{
		self.translation = translation;
		return self;
	}

	pub fn with_origin(mut self, orign: &HexCoord) -> Self
	{
		self.origin = orign.hex.xy();
		return self;
	}

	pub fn with_rotation(mut self, rotation: i32) -> Self
	{
		self.rotation = rotation;
		return self;
	}

	pub fn get_coords(&self) -> Vec<HexCoord>
	{
		let center = HexCoord::from_hex(self.origin);
		return self
			.points
			.iter()
			.map(|p| HexCoord::from_hex(p + self.origin).rotate_around(&center, self.rotation))
			.collect();
	}
}

impl Into<Vec<HexCoord>> for CoordsCollection
{
	fn into(self) -> Vec<HexCoord>
	{
		self.get_coords()
	}
}
