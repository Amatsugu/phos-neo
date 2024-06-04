use bevy::prelude::*;
use world_generation::hex_utils::HexCoord;

#[derive(Resource)]
pub struct BuildingDatabase {
	pub chunks: Vec<BuildingChunk>,
}

impl BuildingDatabase {
	pub fn new(size: UVec2) -> Self {
		let mut db = BuildingDatabase {
			chunks: Vec::with_capacity(size.length_squared() as usize),
		};

		for y in 0..size.y as i32 {
			for x in 0..size.x as i32 {
				let offset = IVec2::new(x, y);
				let index = (x + y * size.x as i32) as usize;
				db.chunks.push(BuildingChunk::new(offset, index));
			}
		}

		return db;
	}

	pub fn get_buildings_in_range(&self, coord: &HexCoord, radius: usize) -> Option<Vec<&BuildingEntry>> {
		assert!(radius != 0, "Radius cannot be zero");
		todo!();
	}

	pub fn get_building(&self, coord: &HexCoord) -> Option<&BuildingEntry> {
		todo!();
	}
}

pub struct BuildingChunk {
	pub entries: Vec<BuildingChunk>,
	pub index: usize,
	pub offset: IVec2,
}

impl BuildingChunk {
	pub fn new(offset: IVec2, index: usize) -> Self {
		return BuildingChunk {
			entries: Vec::new(),
			index,
			offset,
		};
	}

	pub fn get_building(&self, coord: &HexCoord) -> Option<&BuildingEntry> {
		todo!();
	}
}

pub struct BuildingEntry {
	pub coord: HexCoord,
	pub entity: Entity,
}

impl BuildingEntry {
	pub fn new(coord: HexCoord, entity: Entity) -> Self {
		return BuildingEntry { coord, entity };
	}
}
