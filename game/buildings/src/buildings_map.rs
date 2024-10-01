use bevy::prelude::*;
use world_generation::{hex_utils::HexCoord, prelude::Chunk};

#[derive(Resource)]
pub struct BuildingMap {
	pub chunks: Vec<BuildingChunk>,
	pub size: UVec2,
}

impl BuildingMap {
	pub fn new(size: UVec2) -> Self {
		let mut db = BuildingMap {
			size,
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

	pub fn get_buildings_in_range(&self, coord: &HexCoord, radius: usize) -> Vec<&BuildingEntry> {
		assert!(radius != 0, "Radius cannot be zero");

		let w = self.size.x as usize * Chunk::SIZE;
		let h = self.size.y as usize * Chunk::SIZE;
		let coords = coord.hex_select_bounded(radius, true, h, w);
		return self.get_buildings_in_coords(coords);
	}

	pub fn get_buildings_in_coords(&self, coords: Vec<HexCoord>) -> Vec<&BuildingEntry> {
		let mut result = Vec::new();
		for coord in &coords {
			if let Some(buidling) = self.get_building(coord) {
				result.push(buidling);
			}
		}

		return result;
	}

	pub fn get_building(&self, coord: &HexCoord) -> Option<&BuildingEntry> {
		let chunk = &self.chunks[coord.to_chunk_index(self.size.x as usize)];
		return chunk.get_building(coord);
	}

	pub fn add_building(&mut self, entry: BuildingEntry) {
		let chunk = &mut self.chunks[entry.coord.to_chunk_index(self.size.x as usize)];
		chunk.add_building(entry);
	}
}

pub struct BuildingChunk {
	pub entries: Vec<BuildingEntry>,
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
		return self.entries.iter().find(|b| &b.coord == coord);
	}

	pub fn add_building(&mut self, entry: BuildingEntry) {
		self.entries.push(entry);
	}
}

pub struct BuildingEntry {
	pub coord: HexCoord,
	pub entity: Entity,
	pub is_main: bool,
	pub main_entity: Option<Entity>,
	pub has_children: bool,
	pub child_entities: Option<Vec<Entity>>,
}

impl BuildingEntry {
	pub fn new(coord: HexCoord, entity: Entity) -> Self {
		return BuildingEntry {
			coord,
			entity,
			child_entities: None,
			has_children: false,
			main_entity: None,
			is_main: true,
		};
	}

	pub fn new_with_children(coord: HexCoord, entity: Entity, children: Vec<Entity>) -> BuildingEntry {
		return BuildingEntry {
			coord,
			entity,
			child_entities: Some(children),
			has_children: true,
			main_entity: None,
			is_main: true,
		};
	}

	pub fn new_with_parent(coord: HexCoord, entity: Entity, main: Entity) -> BuildingEntry {
		return BuildingEntry {
			coord,
			entity,
			child_entities: None,
			has_children: false,
			main_entity: Some(main),
			is_main: false,
		};
	}
}
