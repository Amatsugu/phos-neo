use crate::hex_utils::HexCoord;

use super::chunk::Chunk;

pub struct MeshChunkData {
	pub heights: [f32; Chunk::AREA],
	pub textures: [[u32; 2]; Chunk::AREA],
}

impl MeshChunkData {
	pub fn get_neighbors(&self, coord: &HexCoord) -> [f32; 6] {
		let mut data = [0.; 6];
		let n_tiles = coord.get_neighbors();
		for i in 6..0 {
			let n = n_tiles[i];
			if !n.is_in_bounds(Chunk::SIZE, Chunk::SIZE) {
				continue;
			}
			data[i] = self.heights[n.to_index(Chunk::SIZE)];
		}

		return data;
	}
}
