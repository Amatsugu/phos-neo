#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{
	asset::Assets,
	ecs::system::Res,
	math::{IVec2, UVec2, Vec3},
	render::mesh::Mesh,
};
use bevy_rapier3d::geometry::{Collider, TriMeshFlags};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use world_generation::{
	biome_painter::BiomePainter,
	generators::{
		chunk_colliders::generate_chunk_collider,
		mesh_generator::{generate_chunk_mesh, generate_chunk_water_mesh},
		packed_mesh_generator::generate_packed_chunk_mesh,
	},
	hex_utils::offset_to_world,
	prelude::{Chunk, Map, MeshChunkData},
	tile_manager::TileAsset,
	tile_mapper::TileMapperAsset,
};

pub fn paint_map(
	map: &mut Map,
	painter: &BiomePainter,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) {
	map.chunks.par_iter_mut().for_each(|chunk: &mut Chunk| {
		paint_chunk(chunk, painter, tiles, mappers);
	});
}

pub fn paint_chunk(
	chunk: &mut Chunk,
	painter: &BiomePainter,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) {
	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let idx = x + z * Chunk::SIZE;
			let height = chunk.heights[idx];
			let biome_id = chunk.biome_id[idx];
			let biome = &painter.biomes[biome_id];
			let mapper = mappers.get(biome.tile_mapper.id());
			let tile_handle = mapper.unwrap().sample_tile(height);
			let tile = tiles.get(tile_handle).unwrap();
			chunk.textures[idx] = [tile.texture_id, tile.side_texture_id];
		}
	}
}

pub fn prepare_chunk_mesh(
	chunk: &MeshChunkData,
	sealevel: f32,
	chunk_offset: IVec2,
	chunk_index: usize,
	map_size: UVec2,
) -> (Mesh, Mesh, (Vec<Vec3>, Vec<[u32; 3]>), Vec3, usize) {
	#[cfg(feature = "tracing")]
	let _gen_mesh = info_span!("Generate Chunk").entered();
	let chunk_mesh = generate_packed_chunk_mesh(chunk);
	let water_mesh = generate_chunk_water_mesh(chunk, sealevel, map_size.x as usize, map_size.y as usize);
	let col_data = generate_chunk_collider(chunk);

	return (
		chunk_mesh,
		water_mesh,
		col_data,
		offset_to_world(chunk_offset * Chunk::SIZE as i32, 0.),
		chunk_index,
	);
}

pub fn prepare_chunk_mesh_with_collider(
	chunk: &MeshChunkData,
	sealevel: f32,
	chunk_offset: IVec2,
	chunk_index: usize,
	map_size: UVec2,
) -> (Mesh, Mesh, Collider, Vec3, usize) {
	let (chunk_mesh, water_mesh, (col_verts, col_indicies), pos, index) =
		prepare_chunk_mesh(chunk, sealevel, chunk_offset, chunk_index, map_size);
	let collider: Collider;
	{
		#[cfg(feature = "tracing")]
		let _collider_span = info_span!("Create Collider Trimesh").entered();
		collider = Collider::trimesh_with_flags(col_verts, col_indicies, TriMeshFlags::DELETE_DUPLICATE_TRIANGLES);
	}
	return (chunk_mesh, water_mesh, collider, pos, index);
}
