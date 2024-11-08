use std::collections::VecDeque;

use bevy::math::IVec2;

use crate::hex_utils::HexCoord;

use super::chunk::Chunk;

pub struct MeshChunkData {
	pub heights: [f32; Chunk::AREA],
	pub textures: [[u32; 2]; Chunk::AREA],
	pub overlay_textures: [Option<u32>; Chunk::AREA],
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

	pub fn caluclate_water_distances(data: &mut Vec<MeshChunkData>, height: usize, width: usize, range: usize) {
		let mut open: VecDeque<(HexCoord, f32, usize)> = VecDeque::new();
		let mut closed: Vec<(HexCoord, f32)> = Vec::new();
		for z in 0..height {
			for x in 0..width {
				let chunk = &mut data[z * height + x];
				chunk.prepare_chunk_open(x * Chunk::SIZE, z * Chunk::SIZE, &mut open);
			}
		}
	}

	fn prepare_chunk_open(&mut self, offset_x: usize, offset_z: usize, open: &mut VecDeque<(HexCoord, f32, usize)>) {
		for z in 0..Chunk::SIZE {
			for x in 0..Chunk::SIZE {
				let coord = HexCoord::from_grid_pos(x + offset_x, z + offset_z);
				let idx = coord.to_chunk_local_index();
				let h = self.heights[idx];
				self.distance_to_land[idx] = if h > self.sealevel { 0.0 } else { 4.0 };
				if h > self.sealevel {
					open.push_back((coord, h, 0));
				}
			}
		}
	}

	fn fill_chunk_borders(
		&mut self,
		chunks: &Vec<MeshChunkData>,
		offset: IVec2,
		open: &mut VecDeque<(HexCoord, f32, usize)>,
		closed: &mut Vec<(HexCoord, f32)>,
	) {
		self.prepare_chunk_open(offset.x as usize * Chunk::SIZE, offset.y as usize * Chunk::SIZE, open);
		todo!("Fill closed list with bordering tiles")
	}
}
