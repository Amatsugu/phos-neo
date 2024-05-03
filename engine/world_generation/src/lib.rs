pub mod biome_painter;
pub mod chunk_colliders;
pub mod heightmap;
pub mod hex_utils;
pub mod mesh_generator;
pub mod packed_mesh_generator;
pub mod tile_manager;
pub mod tile_mapper;

pub mod prelude {
	use crate::hex_utils::{tile_to_world_distance, HexCoord, INNER_RADIUS, OUTER_RADIUS};
	use bevy::math::{IVec2, UVec2, Vec2, Vec3};
	use bevy::prelude::Resource;
	use bevy::prelude::*;
	use bevy::render::mesh::MeshVertexAttribute;
	use bevy::render::render_resource::VertexFormat;
	use bevy_inspector_egui::InspectorOptions;
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

	pub struct Chunk {
		pub heights: [f32; Chunk::SIZE * Chunk::SIZE],
		pub moisture: [f32; Chunk::SIZE * Chunk::SIZE],
		pub temperature: [f32; Chunk::SIZE * Chunk::SIZE],
		pub chunk_offset: IVec2,
	}

	impl Chunk {
		pub const SIZE: usize = 64;
	}

	#[derive(Resource)]
	pub struct Map {
		pub chunks: Vec<Chunk>,
		pub height: usize,
		pub width: usize,
		pub sea_level: f32,
	}

	impl Map {
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

		pub fn get_moisture(&self, pos: &HexCoord) -> f32 {
			let chunk = &self.chunks[pos.to_chunk_index(self.width)];
			return chunk.moisture[pos.to_chunk_local_index()];
		}

		pub fn get_tempurature(&self, pos: &HexCoord) -> f32 {
			let chunk = &self.chunks[pos.to_chunk_index(self.width)];
			return chunk.temperature[pos.to_chunk_local_index()];
		}

		pub fn get_center(&self) -> Vec3 {
			let w = self.width * Chunk::SIZE;
			let h = self.height * Chunk::SIZE;
			return Vec3::new(
				tile_to_world_distance(w as i32 / 2),
				self.sea_level,
				tile_to_world_distance(h as i32 / 2),
			);
		}

		pub fn get_world_width(&self) -> f32 {
			return tile_to_world_distance((self.width * Chunk::SIZE) as i32);
		}
		pub fn get_world_height(&self) -> f32 {
			return tile_to_world_distance((self.height * Chunk::SIZE) as i32);
		}
	}
	pub const ATTRIBUTE_PACKED_VERTEX_DATA: MeshVertexAttribute =
		MeshVertexAttribute::new("PackedVertexData", 988540817, VertexFormat::Uint32);
	pub const ATTRIBUTE_VERTEX_HEIGHT: MeshVertexAttribute =
		MeshVertexAttribute::new("VertexHeight", 988540717, VertexFormat::Float32);

	pub const ATTRIBUTE_TEXTURE_INDEX: MeshVertexAttribute =
		MeshVertexAttribute::new("TextureIndex", 988540917, VertexFormat::Uint32);
}
