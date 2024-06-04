use crate::hex_utils::SHORT_DIAGONAL;
use bevy::prelude::*;

#[derive(Clone)]
pub struct Chunk {
	pub heights: [f32; Chunk::AREA],
	pub textures: [[u32; 2]; Chunk::AREA],
	pub moisture: [f32; Chunk::AREA],
	pub temperature: [f32; Chunk::AREA],
	pub chunk_offset: IVec2,
}

impl Default for Chunk {
	fn default() -> Self {
		Self {
			heights: [0.; Chunk::AREA],
			textures: [[0; 2]; Chunk::AREA],
			moisture: [0.; Chunk::AREA],
			temperature: [0.; Chunk::AREA],
			chunk_offset: Default::default(),
		}
	}
}

impl Chunk {
	pub const SIZE: usize = 64;
	pub const AREA: usize = Chunk::SIZE * Chunk::SIZE;
	pub const WORLD_WIDTH: f32 = Chunk::SIZE as f32 * SHORT_DIAGONAL;
	pub const WORLD_HEIGHT: f32 = Chunk::SIZE as f32 * 1.5;
	pub const WORLD_SIZE: Vec2 = Vec2::new(Chunk::WORLD_WIDTH, Chunk::WORLD_HEIGHT);

	pub fn get_pos_z_edge(&self) -> [f32; Chunk::SIZE] {
		let mut data = [0.; Chunk::SIZE];

		for x in 0..Chunk::SIZE {
			let idx = x + (Chunk::SIZE - 1) * Chunk::SIZE;
			data[x] = self.heights[idx];
		}

		return data;
	}

	pub fn get_neg_z_edge(&self) -> [f32; Chunk::SIZE] {
		let mut data = [0.; Chunk::SIZE];

		for x in 0..Chunk::SIZE {
			data[x] = self.heights[x];
		}

		return data;
	}

	pub fn get_pos_x_edge(&self) -> [f32; Chunk::SIZE] {
		let mut data = [0.; Chunk::SIZE];

		for z in 0..Chunk::SIZE {
			let idx = (Chunk::SIZE - 1) + z * Chunk::SIZE;
			data[z] = self.heights[idx];
		}

		return data;
	}

	pub fn get_neg_x_edge(&self) -> [f32; Chunk::SIZE] {
		let mut data = [0.; Chunk::SIZE];

		for z in 0..Chunk::SIZE {
			let idx = z * Chunk::SIZE;
			data[z] = self.heights[idx];
		}

		return data;
	}
}
