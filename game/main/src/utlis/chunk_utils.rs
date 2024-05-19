#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{asset::Assets, ecs::system::Res, math::Vec3, render::mesh::Mesh};
use bevy_rapier3d::geometry::{Collider, TriMeshFlags};
use world_generation::{
	biome_painter::BiomePainterAsset,
	chunk_colliders::generate_chunk_collider,
	hex_utils::{offset_to_index, offset_to_world},
	mesh_generator::generate_chunk_mesh,
	prelude::{Chunk, Map},
	tile_manager::TileAsset,
	tile_mapper::TileMapperAsset,
};

pub fn prepare_chunk_mesh(
	chunk: &Chunk,
	heightmap: &Map,
	painter: &BiomePainterAsset,
	tile_assets: &Res<Assets<TileAsset>>,
	tile_mappers: &Res<Assets<TileMapperAsset>>,
) -> (Mesh, (Vec<Vec3>, Vec<[u32; 3]>), Vec3, usize) {
	#[cfg(feature = "tracing")]
	let _gen_mesh = info_span!("Generate Chunk").entered();
	let mesh = generate_chunk_mesh(chunk, &heightmap, painter, &tile_assets, &tile_mappers);
	let col_data = generate_chunk_collider(chunk, &heightmap);

	return (
		mesh,
		col_data,
		offset_to_world(chunk.chunk_offset * Chunk::SIZE as i32, 0.),
		offset_to_index(chunk.chunk_offset, heightmap.width),
	);
}

pub fn prepare_chunk_mesh_with_collider(
	chunk: &Chunk,
	heightmap: &Map,
	painter: &BiomePainterAsset,
	tile_assets: &Res<Assets<TileAsset>>,
	tile_mappers: &Res<Assets<TileMapperAsset>>,
) -> (Mesh, Collider, Vec3, usize) {
	let (mesh, (col_verts, col_indicies), pos, index) =
		prepare_chunk_mesh(chunk, heightmap, painter, tile_assets, tile_mappers);
	let collider: Collider;
	{
		#[cfg(feature = "tracing")]
		let _collider_span = info_span!("Create Collider Trimesh").entered();
		collider = Collider::trimesh_with_flags(col_verts, col_indicies, TriMeshFlags::DELETE_DUPLICATE_TRIANGLES);
	}
	return (mesh, collider, pos, index);
}
