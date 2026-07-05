use avian3d::prelude::*;
#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{light::NotShadowCaster, prelude::*};
use hex::prelude::*;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use world_generation::{
	biome_painter::BiomePainter,
	generators::{
		chunk_colliders::generate_chunk_collider,
		mesh_generator::{generate_chunk_mesh, generate_chunk_water_mesh},
	},
	prelude::{Map, MeshChunkData},
	tile_manager::TileAsset,
	tile_mapper::TileMapperAsset,
};

use crate::{map_rendering::render_distance_system::RenderDistanceVisibility, prelude::PhosChunk};

pub fn paint_map(
	map: &mut Map,
	painter: &BiomePainter,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
)
{
	map.chunks.par_iter_mut().for_each(|chunk: &mut Chunk| {
		paint_chunk(chunk, painter, tiles, mappers);
	});
}

pub fn paint_chunk(
	chunk: &mut Chunk,
	painter: &BiomePainter,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
)
{
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

pub fn prepare_chunk_mesh(chunk: &MeshChunkData, sealevel: f32, map_size: UVec2) -> (Mesh, Option<Mesh>)
{
	#[cfg(feature = "tracing")]
	let _gen_mesh = info_span!("Generate Chunk Mesh").entered();
	let chunk_mesh = generate_chunk_mesh(chunk);
	let water_mesh = generate_chunk_water_mesh(chunk, sealevel, map_size.x as usize, map_size.y as usize);

	return (chunk_mesh, water_mesh);
}

pub fn prepare_chunk_mesh_with_collider(
	chunk: &MeshChunkData,
	sealevel: f32,
	map_size: UVec2,
) -> (Mesh, Option<Mesh>, Collider)
{
	let (chunk_mesh, water_mesh) = prepare_chunk_mesh(chunk, sealevel, map_size);
	let collider: Collider;
	{
		#[cfg(feature = "tracing")]
		let _collider_span = info_span!("Create Collider Trimesh").entered();
		let (col_verts, col_indicies) = generate_chunk_collider(chunk);
		collider = Collider::trimesh(col_verts, col_indicies);
	}
	return (chunk_mesh, water_mesh, collider);
}

pub fn create_water_chunk(
	position: Vec3,
	index: usize,
	water_mesh: Handle<Mesh>,
	water_material: Handle<impl bevy::pbr::Material>,
) -> impl Bundle
{
	(
		Mesh3d(water_mesh),
		MeshMaterial3d(water_material),
		Transform::from_translation(position),
		Name::new(format!("Water {}", index)),
		PhosChunk::new(index),
		NotShadowCaster,
		RenderDistanceVisibility::chunk_centered(),
	)
}
