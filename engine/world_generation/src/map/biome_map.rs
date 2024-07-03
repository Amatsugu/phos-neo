use bevy::math::UVec2;

use super::chunk::Chunk;

pub struct BiomeMap {
	pub height: usize,
	pub width: usize,
	pub chunks: Vec<[Vec<f32>; Chunk::AREA]>,
}

impl BiomeMap {
	pub fn new(size: UVec2) -> Self {
		return BiomeMap {
			height: size.y as usize,
			width: size.x as usize,
			chunks: Vec::with_capacity(size.length_squared() as usize),
		};
	}

	pub fn blend(&mut self, count: usize) {
		for _ in 0..count {
			self.blend_once();
		}
	}

	fn blend_once(&mut self) {
		for y in 0..self.height {
			for x in 0..self.width {
				let kernel = self.get_kernel(x as i32, y as i32);
				let r = kernel.iter().filter_map(|f| *f).fold(Vec::new(), |a, b| {
					if a.len() != b.len() {
						return b.iter().map(|f| *f).collect();
					}
					return a.iter().zip(b).map(|v| v.0 + v.1).collect();
				});

				let sum: f32 = r.iter().sum();

				*self.get_biome_mut(x, y) = r.iter().map(|f| f / sum).collect();
			}
		}
	}

	fn get_biome_mut(&mut self, x: usize, y: usize) -> &mut Vec<f32> {
		let cx = x as usize / Chunk::SIZE;
		let cy = y as usize / Chunk::SIZE;
		let chunk_index = cy * self.width as usize + cx;
		let ix = x as usize - (x as usize * Chunk::SIZE);
		let iy = y as usize - (y as usize * Chunk::SIZE);
		let index = iy * self.width as usize + ix;
		return &mut self.chunks[chunk_index][index];
	}

	pub fn get_kernel(&self, x: i32, y: i32) -> [Option<&Vec<f32>>; 9] {
		return [
			self.get_biome(x - 1, y - 1),
			self.get_biome(x, y - 1),
			self.get_biome(x + 1, y - 1),
			self.get_biome(x - 1, y),
			self.get_biome(x, y),
			self.get_biome(x + 1, y),
			self.get_biome(x - 1, y + 1),
			self.get_biome(x, y + 1),
			self.get_biome(x + 1, y + 1),
		];
	}

	pub fn get_biome(&self, x: i32, y: i32) -> Option<&Vec<f32>> {
		if x < 0 || y < 0 {
			return None;
		}
		if x >= self.width as i32 || y >= self.height as i32 {
			return None;
		}

		let cx = x as usize / Chunk::SIZE;
		let cy = y as usize / Chunk::SIZE;
		let chunk_index = cy * self.width as usize + cx;
		let ix = x as usize - (x as usize * Chunk::SIZE);
		let iy = y as usize - (y as usize * Chunk::SIZE);
		let index = iy * self.width as usize + ix;
		return Some(&self.chunks[chunk_index][index]);
	}
}
