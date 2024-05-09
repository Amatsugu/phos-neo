use bevy::prelude::*;
use bevy_xpbd_3d::plugins::collision::Collider;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use world_generation::{
	biome_painter::BiomePainterAsset,
	chunk_colliders::generate_chunk_collider,
	hex_utils::{self, offset_to_world, SHORT_DIAGONAL},
	mesh_generator::generate_chunk_mesh,
	prelude::{Chunk, Map},
	tile_manager::TileAsset,
	tile_mapper::TileMapperAsset,
};

use crate::{
	prelude::{ChunkAtlas, PhosChunk, PhosChunkRegistry},
	utlis::render_distance_system::RenderDistanceVisibility,
};

use super::prelude::CurrentBiomePainter;
pub struct ChunkRebuildPlugin;

impl Plugin for ChunkRebuildPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ChunkRebuildQueue::default());
		app.init_resource::<PhosChunkRegistry>();
		app.add_systems(PreUpdate, chunk_rebuilder);
	}
}

#[derive(Resource, Default)]
pub struct ChunkRebuildQueue {
	pub queue: Vec<usize>,
}

fn chunk_rebuilder(
	mut commands: Commands,
	mut queue: ResMut<ChunkRebuildQueue>,
	mut chunks: ResMut<PhosChunkRegistry>,
	atlas: Res<ChunkAtlas>,
	heightmap: Res<Map>,
	tile_assets: Res<Assets<TileAsset>>,
	tile_mappers: Res<Assets<TileMapperAsset>>,
	mut meshes: ResMut<Assets<Mesh>>,
	biome_painters: Res<Assets<BiomePainterAsset>>,
	painter: Res<CurrentBiomePainter>,
) {
	if queue.queue.len() == 0 {
		return;
	}
	queue.queue.dedup();

	for chunk_index in &queue.queue {
		let chunk = chunks.chunks[*chunk_index];
		commands.entity(chunk).despawn();
	}

	let b_painter = biome_painters.get(painter.handle.clone());

	let cur_painter = b_painter.unwrap();

	let chunk_meshes: Vec<_> = queue
		.queue
		.par_iter()
		.map(|idx| {
			let chunk = &heightmap.chunks[*idx];
			let mesh = generate_chunk_mesh(chunk, &heightmap, cur_painter, &tile_assets, &tile_mappers);
			let (col_verts, col_indicies) = generate_chunk_collider(chunk, &heightmap);
			let collider = Collider::trimesh(col_verts, col_indicies);
			return (
				mesh,
				collider,
				offset_to_world(chunk.chunk_offset * Chunk::SIZE as i32, 0.),
				hex_utils::offset_to_index(chunk.chunk_offset, heightmap.width),
			);
		})
		.collect();

	for (mesh, collider, pos, index) in chunk_meshes {
		let chunk = commands.spawn((
			MaterialMeshBundle {
				mesh: meshes.add(mesh),
				material: atlas.chunk_material_handle.clone(),
				transform: Transform::from_translation(pos),
				..default()
			},
			PhosChunk::new(index),
			RenderDistanceVisibility::default().with_offset(Vec3::new(
				(Chunk::SIZE / 2) as f32 * SHORT_DIAGONAL,
				0.,
				(Chunk::SIZE / 2) as f32 * 1.5,
			)),
			collider,
		));
		chunks.chunks[index] = chunk.id();
	}
	queue.queue.clear();
}
