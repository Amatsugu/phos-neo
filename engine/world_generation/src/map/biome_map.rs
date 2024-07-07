use bevy::math::{UVec2, Vec3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::chunk::Chunk;

pub struct BiomeMap {
	pub height: usize,
	pub width: usize,
	pub biome_count: usize,
	pub chunks: Vec<BiomeChunk>,
}

#[derive(Default, Clone, Copy)]
pub struct BiomeData {
	pub moisture: f32,
	pub temperature: f32,
	pub continentality: f32,
}

impl Into<Vec3> for &BiomeData {
	fn into(self) -> Vec3 {
		return Vec3::new(self.moisture, self.temperature, self.continentality);
	}
}

impl Into<Vec3> for BiomeData {
	fn into(self) -> Vec3 {
		return Vec3::new(self.moisture, self.temperature, self.continentality);
	}
}

impl BiomeMap {
	pub fn new(size: UVec2, biome_count: usize) -> Self {
		let len = size.x as usize * size.y as usize * Chunk::AREA;
		return BiomeMap {
			height: size.y as usize * Chunk::SIZE,
			width: size.x as usize * Chunk::SIZE,
			biome_count,
			chunks: Vec::with_capacity(len),
		};
	}

	pub fn blend(&mut self, count: usize) {
		assert!(count != 0, "Count cannot be 0");
		for _ in 0..count {
			self.blend_once();
		}
	}

	fn blend_once(&mut self) {
		let c: Vec<BiomeChunk> = (0..self.chunks.len())
			.into_par_iter()
			.map(|i| &self.chunks[i])
			.map(|chunk| {
				let tiles: Vec<_> = (0..Chunk::SIZE)
					.into_par_iter()
					.map(|y| {
						let mut new_tiles = Vec::with_capacity(self.width);
						for x in 0..Chunk::SIZE {
							let kernel = self.get_kernel(x as i32, y as i32);
							let r = kernel
								.iter()
								.filter_map(|f| *f)
								.fold(vec![0.; self.biome_count], |a, b| {
									return a.iter().zip(b).map(|v| v.0 + v.1).collect();
								});

							let sum: f32 = r.iter().sum();
							if sum == 0. {
								new_tiles.push(vec![0.; self.biome_count]);
								continue;
							}
							new_tiles.push(r.iter().map(|f| f / sum).collect());
						}
						return new_tiles;
					})
					.flatten()
					.collect();
				return BiomeChunk {
					tiles,
					data: chunk.data,
				};
			})
			.collect();
		self.chunks = c;
	}

	fn get_kernel(&self, x: i32, y: i32) -> [Option<&Vec<f32>>; 9] {
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

		let chunk = &self.chunks[cx + cy * Chunk::SIZE];

		return Some(chunk.get_biome(x as usize - cx * Chunk::SIZE, y as usize - cy * Chunk::SIZE));
	}

	pub fn get_biome_data(&self, x: usize, y: usize) -> &BiomeData {
		let cx = x / Chunk::SIZE;
		let cy = y / Chunk::SIZE;

		let chunk = &self.chunks[cx + cy * Chunk::SIZE];

		return chunk.get_biome_data(x - cx * Chunk::SIZE, y - cy * Chunk::SIZE);
	}
}

#[derive(Clone)]
pub struct BiomeChunk {
	pub tiles: Vec<Vec<f32>>,
	pub data: [BiomeData; Chunk::AREA],
}

impl BiomeChunk {
	pub fn get_biome(&self, x: usize, y: usize) -> &Vec<f32> {
		return &self.tiles[x as usize + y as usize * Chunk::SIZE];
	}

	pub fn get_biome_data(&self, x: usize, y: usize) -> &BiomeData {
		return &self.data[x + y * Chunk::SIZE];
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn biome_blend() {
		let mut biome = BiomeMap::new(UVec2::ONE * 16, 8);
		let w = biome.width / Chunk::SIZE;
		let h = biome.height / Chunk::SIZE;

		for y in 0..h {
			for x in 0..w {
				let mut b = vec![0.; biome.biome_count];
				let idx = (x + y) % biome.biome_count;
				b[idx] = 1.;
				biome.chunks.push(generate_chunk(x, y, b));
			}
		}

		biome.blend(8);
	}

	fn generate_chunk(x: usize, y: usize, biome: Vec<f32>) -> BiomeChunk {
		let chunk = BiomeChunk {
			data: [BiomeData::default(); Chunk::AREA],
			tiles: (0..Chunk::AREA).into_iter().map(|_| biome.clone()).collect(),
		};

		return chunk;
	}
}
