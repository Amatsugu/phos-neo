use bevy::prelude::*;

use crate::hex_utils::*;

use super::{
	biome_map::{BiomeData, BiomeMap},
	chunk::Chunk,
	mesh_chunk::MeshChunkData,
};

#[derive(Resource, Clone)]
pub struct Map {
	pub chunks: Vec<Chunk>,
	pub height: usize,
	pub width: usize,
	pub sea_level: f32,
	pub min_level: f32,
	pub max_level: f32,
	pub biome_count: usize,
}

impl Map {
	pub fn get_tile_count(&self) -> usize {
		return self.get_tile_width() * self.get_tile_height();
	}

	pub fn get_tile_width(&self) -> usize {
		return self.width * Chunk::SIZE;
	}

	pub fn get_tile_height(&self) -> usize {
		return self.height * Chunk::SIZE;
	}

	pub fn get_chunk_mesh_data(&self, chunk_index: usize) -> MeshChunkData {
		#[cfg(feature = "tracing")]
		let _spawn_span = info_span!("Chunk Mesh Data").entered();
		let chunk = &self.chunks[chunk_index];

		return MeshChunkData {
			min_height: self.min_level,
			heights: chunk.heights.clone(),
			textures: chunk.textures.clone(),
		};
	}

	pub fn get_neighbors(&self, pos: &HexCoord) -> [Option<f32>; 6] {
		let mut results: [Option<f32>; 6] = [None; 6];
		let w = self.width * Chunk::SIZE;
		let h = self.height * Chunk::SIZE;
		let n_tiles = pos.get_neighbors();
		for i in 0..6 {
			let n_tile = n_tiles[i];
			if !n_tile.is_in_bounds(h, w) {
				continue;
			}
			let c_idx = n_tile.to_chunk_index(self.width);
			let chunk = &self.chunks[c_idx];
			let local = n_tile.to_chunk_local_index();
			results[i] = Some(chunk.heights[local]);
		}
		return results;
	}

	pub fn sample_height(&self, pos: &HexCoord) -> f32 {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &self.chunks[pos.to_chunk_index(self.width)];
		return chunk.heights[pos.to_chunk_local_index()];
	}

	pub fn sample_height_mut(&mut self, pos: &HexCoord) -> &mut f32 {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &mut self.chunks[pos.to_chunk_index(self.width)];
		return &mut chunk.heights[pos.to_chunk_local_index()];
	}

	pub fn is_in_bounds(&self, pos: &HexCoord) -> bool {
		return pos.is_in_bounds(self.height * Chunk::SIZE, self.width * Chunk::SIZE);
	}

	pub fn get_biome_id(&self, pos: &HexCoord) -> usize {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &self.chunks[pos.to_chunk_index(self.width)];
		return chunk.biome_id[pos.to_chunk_local_index()];
	}
	/*
	pub fn get_biome_noise(&self, pos: &HexCoord) -> &BiomeData {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &self.chunks[pos.to_chunk_index(self.width)];
		return &chunk.biome_data[pos.to_chunk_local_index()];
	}

	pub fn get_moisture(&self, pos: &HexCoord) -> f32 {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &self.chunks[pos.to_chunk_index(self.width)];
		return chunk.biome_data[pos.to_chunk_local_index()].moisture;
	}

	pub fn get_tempurature(&self, pos: &HexCoord) -> f32 {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &self.chunks[pos.to_chunk_index(self.width)];
		return chunk.biome_data[pos.to_chunk_local_index()].temperature;
	}

	pub fn get_continentality(&self, pos: &HexCoord) -> f32 {
		assert!(
			self.is_in_bounds(pos),
			"The provided coordinate is not within the map bounds"
		);

		let chunk = &self.chunks[pos.to_chunk_index(self.width)];
		return chunk.biome_data[pos.to_chunk_local_index()].continentality;
	}
	*/

	pub fn get_center(&self) -> Vec3 {
		let w = self.get_world_width();
		let h = self.get_world_height();
		return Vec3::new(w / 2., self.sea_level, h / 2.);
	}

	pub fn get_world_width(&self) -> f32 {
		return (self.width * Chunk::SIZE) as f32 * SHORT_DIAGONAL;
	}
	pub fn get_world_height(&self) -> f32 {
		return (self.height * Chunk::SIZE) as f32 * 1.5;
	}

	pub fn get_world_size(&self) -> Vec2 {
		return Vec2::new(self.get_world_width(), self.get_world_height());
	}

	pub fn set_height(&mut self, pos: &HexCoord, height: f32) {
		self.chunks[pos.to_chunk_index(self.width)].heights[pos.to_chunk_local_index()] = height;
	}

	pub fn create_crater(&mut self, pos: &HexCoord, radius: usize, depth: f32) -> Vec<(HexCoord, f32)> {
		assert!(radius != 0, "Radius cannot be zero");

		let tiles = self.hex_select_mut(pos, radius, true, |p, h, r| {
			let d = (r as f32) / (radius as f32);
			let cur = *h;
			let h2 = cur - depth;
			*h = h2.lerp(cur, d * d).max(0.);

			return (*p, *h);
		});

		return tiles;
	}

	pub fn hex_select<OP, Ret>(&self, center: &HexCoord, radius: usize, include_center: bool, op: OP) -> Vec<Ret>
	where
		OP: (Fn(&HexCoord, f32, usize) -> Ret) + Sync + Send,
	{
		assert!(radius != 0, "Radius cannot be zero");

		let mut result = if include_center {
			Vec::with_capacity(get_tile_count_in_range(radius) + 1)
		} else {
			Vec::with_capacity(get_tile_count_in_range(radius))
		};
		if include_center {
			let h = self.sample_height(&center);
			result.push((op)(center, h, 0));
		}

		for k in 0..(radius + 1) {
			let mut p = center.scale(4, k);
			for i in 0..6 {
				for _j in 0..k {
					p = p.get_neighbor(i);
					if self.is_in_bounds(&p) {
						let h = self.sample_height(&p);
						result.push((op)(&p, h, k));
					}
				}
			}
		}

		return result;
	}

	pub fn hex_select_mut<OP, Ret>(
		&mut self,
		center: &HexCoord,
		radius: usize,
		include_center: bool,
		op: OP,
	) -> Vec<Ret>
	where
		OP: (Fn(&HexCoord, &mut f32, usize) -> Ret) + Sync + Send,
	{
		assert!(radius != 0, "Radius cannot be zero");

		let mut result = if include_center {
			Vec::with_capacity(get_tile_count_in_range(radius) + 1)
		} else {
			Vec::with_capacity(get_tile_count_in_range(radius))
		};
		if include_center {
			let h = self.sample_height_mut(&center);
			result.push((op)(center, h, 0));
		}

		for k in 0..(radius + 1) {
			let mut p = center.scale(4, k);
			for i in 0..6 {
				for _j in 0..k {
					p = p.get_neighbor(i);
					if self.is_in_bounds(&p) {
						let h = self.sample_height_mut(&p);
						result.push((op)(&p, h, k));
					}
				}
			}
		}

		return result;
	}
}
