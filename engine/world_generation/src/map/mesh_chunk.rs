use crate::hex_utils::HexCoord;

use super::chunk::Chunk;

pub struct MeshChunkData {
	pub heights: [f32; Chunk::AREA],
	pub textures: [[u32; 2]; Chunk::AREA],
	pub min_height: f32,
	pub sealevel: f32,
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

	pub fn get_neighbors_map_bounded(&self, coord: &HexCoord, width: usize, height: usize) -> [f32; 6] {
		let mut data = [self.min_height; 6];
		let n_tiles = coord.get_neighbors();
		for i in 0..6 {
			let n = n_tiles[i];
			if !n.is_in_bounds(Chunk::SIZE * width, Chunk::SIZE * height) {
				continue;
			}
			data[i] = self.heights[n.to_index(Chunk::SIZE)];
		}

		return data;
	}
}
