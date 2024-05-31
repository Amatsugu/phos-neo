pub mod biome_painter;
pub mod chunk_colliders;
pub mod heightmap;
pub mod hex_utils;
pub mod mesh_generator;
pub mod packed_mesh_generator;
pub mod tile_manager;
pub mod tile_mapper;

pub mod prelude {
	use crate::hex_utils::{get_tile_count, HexCoord, INNER_RADIUS, OUTER_RADIUS, SHORT_DIAGONAL};
	use bevy::math::{IVec2, UVec2, Vec2, Vec3};
	use bevy::prelude::Resource;
	use bevy::prelude::*;
	use bevy::render::mesh::MeshVertexAttribute;
	use bevy::render::render_resource::VertexFormat;
	use bevy_inspector_egui::InspectorOptions;
	use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
	pub const TEX_MULTI: Vec2 = Vec2::new(1000., 1.);

	pub const HEX_CORNERS: [Vec3; 6] = [
		Vec3::new(0., 0., OUTER_RADIUS),
		Vec3::new(INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
		Vec3::new(INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
		Vec3::new(0., 0., -OUTER_RADIUS),
		Vec3::new(-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
		Vec3::new(-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
	];

	pub const HEX_NORMALS: [Vec3; 6] = [
		Vec3::new(INNER_RADIUS / 2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / 2.),
		Vec3::Z,
		Vec3::new(INNER_RADIUS / -2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / 2.),
		Vec3::new(INNER_RADIUS / -2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / -2.),
		Vec3::NEG_Z,
		Vec3::new(INNER_RADIUS / 2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / -2.),
	];

	#[derive(Resource, Reflect, Default)]
	#[reflect(Resource)]
	pub struct GenerationConfig {
		pub noise_scale: f64,
		pub sea_level: f64,
		pub border_size: f32,
		pub size: UVec2,
		pub layers: Vec<GeneratorLayer>,
	}

	impl GenerationConfig {
		pub fn get_total_width(&self) -> usize {
			return self.size.x as usize * Chunk::SIZE;
		}
		pub fn get_total_height(&self) -> usize {
			return self.size.y as usize * Chunk::SIZE;
		}
	}

	#[derive(Reflect, InspectorOptions)]
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

	pub struct MeshChunkData {
		pub heights: [f32; Chunk::AREA],
		pub textures: [[u32; 2]; Chunk::AREA],
	}

	impl MeshChunkData {
		pub fn get_neighbors(&self, coord: &HexCoord) -> [f32; 6] {
			let mut data = [0.; 6];
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
	}

	#[derive(Resource, Clone)]
	pub struct Map {
		pub chunks: Vec<Chunk>,
		pub height: usize,
		pub width: usize,
		pub sea_level: f32,
	}

	impl Map {
		pub fn get_chunk_mesh_data(&self, chunk_index: usize) -> MeshChunkData {
			#[cfg(feature = "tracing")]
			let _spawn_span = info_span!("Chunk Mesh Data").entered();
			let chunk = &self.chunks[chunk_index];

			return MeshChunkData {
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
			let chunk = &self.chunks[pos.to_chunk_index(self.width)];
			return chunk.heights[pos.to_chunk_local_index()];
		}

		pub fn sample_height_mut(&mut self, pos: &HexCoord) -> &mut f32 {
			let chunk = &mut self.chunks[pos.to_chunk_index(self.width)];
			return &mut chunk.heights[pos.to_chunk_local_index()];
		}

		pub fn is_in_bounds(&self, pos: &HexCoord) -> bool {
			return pos.is_in_bounds(self.height * Chunk::SIZE, self.width * Chunk::SIZE);
		}

		pub fn get_moisture(&self, pos: &HexCoord) -> f32 {
			let chunk = &self.chunks[pos.to_chunk_index(self.width)];
			return chunk.moisture[pos.to_chunk_local_index()];
		}

		pub fn get_tempurature(&self, pos: &HexCoord) -> f32 {
			let chunk = &self.chunks[pos.to_chunk_index(self.width)];
			return chunk.temperature[pos.to_chunk_local_index()];
		}

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

		pub fn create_crater(&mut self, pos: &HexCoord, radius: usize, depth: f32) -> Vec<usize> {
			assert!(radius != 0, "Radius cannot be zero");
			let width = self.width;

			let mut chunks = self.hex_select_mut(pos, radius, true, |p, h, r| {
				let d = (r as f32) / (radius as f32);
				let cur = *h;
				let h2 = cur - depth;
				*h = h2.lerp(cur, d * d);

				return p.to_chunk_index(width);
			});

			chunks.dedup();

			return chunks;
		}

		pub fn hex_select<OP, Ret>(&self, center: &HexCoord, radius: usize, include_center: bool, op: OP) -> Vec<Ret>
		where
			OP: Fn(&HexCoord, f32, usize) -> Ret + Sync + Send,
		{
			assert!(radius != 0, "Radius cannot be zero");

			if include_center {
				let h = self.sample_height(&center);
				(op)(&center, h, 0);
			}

			let mut result = Vec::with_capacity(get_tile_count(radius));

			for k in 0..(radius + 1) {
				let mut p = center.scale(4, k);
				for i in 0..6 {
					for _j in 0..k {
						p = p.get_neighbor(i);
						let h = self.sample_height(&p);
						result.push((op)(&p, h, k));
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
			OP: Fn(&HexCoord, &mut f32, usize) -> Ret + Sync + Send,
		{
			assert!(radius != 0, "Radius cannot be zero");

			if include_center {
				let h = self.sample_height_mut(&center);
				(op)(&center, h, 0);
			}

			let mut result = Vec::with_capacity(get_tile_count(radius));

			for k in 0..(radius + 1) {
				let mut p = center.scale(4, k);
				for i in 0..6 {
					for _j in 0..k {
						p = p.get_neighbor(i);
						let h = self.sample_height_mut(&p);
						result.push((op)(&p, h, k));
					}
				}
			}

			return result;
		}
	}
	pub const ATTRIBUTE_PACKED_VERTEX_DATA: MeshVertexAttribute =
		MeshVertexAttribute::new("PackedVertexData", 988540817, VertexFormat::Uint32);
	pub const ATTRIBUTE_VERTEX_HEIGHT: MeshVertexAttribute =
		MeshVertexAttribute::new("VertexHeight", 988540717, VertexFormat::Float32);

	pub const ATTRIBUTE_TEXTURE_INDEX: MeshVertexAttribute =
		MeshVertexAttribute::new("TextureIndex", 988540917, VertexFormat::Uint32);
}
