pub mod prelude {
	use crate::hex_utils::HexCoord;
	use bevy::math::IVec2;
	use bevy::prelude::Resource;

	pub struct GenerationConfig {
		pub noise_scale: f64,
		pub sea_level: f64,
		pub layers: Vec<GeneratorLayer>,
	}
	pub struct GeneratorLayer {
		pub strength: f64,
		pub min_value: f64,
		pub base_roughness: f64,
		pub roughness: f64,
		pub persistence: f64,
		pub is_rigid: bool,
		pub weight: f64,
		pub weight_multi: f64,
		pub layers: usize,
		pub first_layer_mask: bool,
	}
	pub struct Chunk {
		pub points: Vec<f32>,
		pub chunk_offset: IVec2,
	}

	impl Chunk {
		pub const SIZE: usize = 32;
	}

	#[derive(Resource)]
	pub struct Map {
		pub chunks: Vec<Chunk>,
		pub height: usize,
		pub width: usize,
	}

	impl Map {
		pub fn get_neighbors(&self, pos: &HexCoord) -> [Option<f32>; 6] {
			let mut results: [Option<f32>; 6] = [None; 6];
			let n_tiles = pos.get_neighbors();
			for i in 0..6 {
				let n_tile = n_tiles[i];
				if !n_tile.is_in_bounds(self.height, self.width) {
					continue;
				}
				let c_idx = n_tile.to_chunk_index(self.width);
				let chunk = &self.chunks[c_idx as usize];
				let local = n_tile.to_chunk_local_index();
				results[i] = (Some(chunk.points[local as usize]));
			}
			return results;
		}
	}
}

pub mod heightmap;
pub mod hex_utils;
pub mod mesh_generator;
