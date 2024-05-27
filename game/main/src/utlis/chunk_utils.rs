#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{asset::Assets, ecs::system::Res, math::Vec3, render::mesh::Mesh};
use bevy_rapier3d::geometry::{Collider, TriMeshFlags};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use world_generation::{
	biome_painter::BiomePainterAsset,
	chunk_colliders::generate_chunk_collider,
	hex_utils::{offset_to_index, offset_to_world},
	mesh_generator::generate_chunk_mesh,
	prelude::{Chunk, Map},
	tile_manager::TileAsset,
	tile_mapper::TileMapperAsset,
};

pub fn paint_map(
	map: &mut Map,
	painter: &BiomePainterAsset,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) {
	map.chunks.par_iter_mut().for_each(|chunk: &mut Chunk| {
		paint_chunk(chunk, painter, tiles, mappers);
	});
}

pub fn paint_chunk(
	chunk: &mut Chunk,
	painter: &BiomePainterAsset,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) {
	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let idx = x + z * Chunk::SIZE;
			let height = chunk.heights[idx];
			let moisture = chunk.moisture[idx];
			let temperature = chunk.temperature[idx];
			let biome = mappers.get(painter.sample_biome(moisture, temperature));
			let tile_handle = biome.unwrap().sample_tile(height);
			let tile = tiles.get(tile_handle).unwrap();
			chunk.textures[idx] = [tile.texture_id, tile.side_texture_id];
		}
	}
}

pub fn prepare_chunk_mesh(chunk: &Chunk, heightmap: &Map) -> (Mesh, (Vec<Vec3>, Vec<[u32; 3]>), Vec3, usize) {
	#[cfg(feature = "tracing")]
	let _gen_mesh = info_span!("Generate Chunk").entered();
	let mesh = generate_chunk_mesh(chunk, &heightmap);
	let col_data = generate_chunk_collider(chunk, &heightmap);

	return (
		mesh,
		col_data,
		offset_to_world(chunk.chunk_offset * Chunk::SIZE as i32, 0.),
		offset_to_index(chunk.chunk_offset, heightmap.width),
	);
}

pub fn prepare_chunk_mesh_with_collider(chunk: &Chunk, heightmap: &Map) -> (Mesh, Collider, Vec3, usize) {
	let (mesh, (col_verts, col_indicies), pos, index) = prepare_chunk_mesh(chunk, heightmap);
	let collider: Collider;
	{
		#[cfg(feature = "tracing")]
		let _collider_span = info_span!("Create Collider Trimesh").entered();
		collider = Collider::trimesh_with_flags(col_verts, col_indicies, TriMeshFlags::DELETE_DUPLICATE_TRIANGLES);
	}
	return (mesh, collider, pos, index);
}
