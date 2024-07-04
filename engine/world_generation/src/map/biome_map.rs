use bevy::math::UVec2;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::chunk::Chunk;

pub struct BiomeMap {
	pub height: usize,
	pub width: usize,
	pub biome_count: usize,
	pub tiles: Vec<Vec<f32>>,
}

impl BiomeMap {
	pub fn new(size: UVec2, biome_count: usize) -> Self {
		return BiomeMap {
			height: size.y as usize * Chunk::SIZE,
			width: size.x as usize * Chunk::SIZE,
			biome_count,
			tiles: Vec::with_capacity(size.x as usize * size.y as usize * Chunk::AREA),
		};
	}

	pub fn blend(&mut self, count: usize) {
		assert!(count != 0, "Count cannot be 0");
		for _ in 0..count {
			self.blend_once();
		}
	}

	fn blend_once(&mut self) {
		let t: Vec<_> = (0..self.height)
			.into_par_iter()
			.map(|y| {
				let mut new_tiles = Vec::with_capacity(self.width);
				for x in 0..self.width {
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

		self.tiles = t;
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

		return Some(&self.tiles[x as usize + y as usize * self.width]);
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn biome_blend() {
		let mut biome = BiomeMap::new(UVec2::ONE * 16, 8);

		for y in 0..biome.height {
			for x in 0..biome.width {
				let mut b = vec![0.; biome.biome_count];
				let i = x / Chunk::SIZE;
				let z = y / Chunk::SIZE;
				let idx = (i + z) % biome.biome_count;
				b[idx] = 1.;
				biome.tiles.push(b);
			}
		}

		biome.blend(8);
	}
}
