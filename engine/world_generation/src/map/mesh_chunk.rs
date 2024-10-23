use crate::hex_utils::HexCoord;

use super::chunk::Chunk;

pub struct MeshChunkData {
	pub heights: [f32; Chunk::AREA],
	pub textures: [[u32; 2]; Chunk::AREA],
	pub min_height: f32,
	pub sealevel: f32,
	pub distance_to_land: [f32; Chunk::AREA],
}

impl MeshChunkData {
	pub fn get_neighbors(&self, coord: &HexCoord) -> [f32; 6] {
		let mut data = [self.min_height; 6];
		let n_tiles = coord.get_neighbors();
		for i in 0..6 {
			let n = n_tiles[i];
			if !n.is_in_bounds(Chunk::SIZE, Chunk::SIZE) {
				continue;
			}
			data[i] = self.heights[n.to_index(Chunk::SIZE)];
		}

		return data;
	}

	pub fn get_neighbors_with_water_info(&self, coord: &HexCoord) -> ([(f32, Option<f32>); 6], bool) {
		let mut has_land = false;
		let mut data = [(self.min_height, None); 6];
		let n_tiles = coord.get_neighbors();
		for i in 0..6 {
			let n = n_tiles[i];
			if !n.is_in_bounds(Chunk::SIZE, Chunk::SIZE) {
				continue;
			}
			let idx = n.to_index(Chunk::SIZE);
			data[i] = (self.heights[idx], Some(self.distance_to_land[idx]));
			if data[i].0 > self.sealevel {
				has_land = true;
			}
		}
		return (data, has_land);
	}
}
