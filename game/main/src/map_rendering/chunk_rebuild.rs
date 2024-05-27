use std::thread;

use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::geometry::TriMeshFlags;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use world_generation::{
	biome_painter::BiomePainterAsset,
	hex_utils::SHORT_DIAGONAL,
	prelude::{Chunk, Map},
	tile_manager::TileAsset,
	tile_mapper::TileMapperAsset,
};

use crate::{
	prelude::{ChunkAtlas, PhosChunk, PhosChunkRegistry},
	utlis::{chunk_utils::prepare_chunk_mesh, render_distance_system::RenderDistanceVisibility},
};

use super::prelude::CurrentBiomePainter;
pub struct ChunkRebuildPlugin;

impl Plugin for ChunkRebuildPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ChunkRebuildQueue::default());
		app.init_resource::<PhosChunkRegistry>();
		app.add_systems(PreUpdate, chunk_rebuilder);
		app.add_systems(PreUpdate, collider_task_resolver);
	}
}

#[derive(Resource, Default)]
pub struct ChunkRebuildQueue {
	pub queue: Vec<usize>,
}

//Todo: Re-use existing entity/collider until new collider is generated
fn chunk_rebuilder(
	mut commands: Commands,
	mut queue: ResMut<ChunkRebuildQueue>,
	mut chunks: ResMut<PhosChunkRegistry>,
	atlas: Res<ChunkAtlas>,
	heightmap: Res<Map>,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	if queue.queue.len() == 0 {
		return;
	}
	queue.queue.dedup();

	let chunk_indices = queue.queue.clone();
	let pool = AsyncComputeTaskPool::get();

	for chunk_index in &queue.queue {
		let chunk = chunks.chunks[*chunk_index];
		// commands.entity(chunk).remove::<Handle<Mesh>>();
		commands.entity(chunk).despawn();
	}

	let chunk_meshes: Vec<_> = queue
		.queue
		.par_iter()
		.map(|idx| {
			return prepare_chunk_mesh(&heightmap.chunks[*idx], &heightmap);
		})
		.collect();

	for (mesh, collider_data, pos, index) in chunk_meshes {
		let mut chunk = commands.spawn((
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
		));
		let entity = chunk.id();
		let task = pool.spawn(async move {
			let mut queue = CommandQueue::default();
			let c = Collider::trimesh_with_flags(
				collider_data.0,
				collider_data.1,
				TriMeshFlags::DELETE_DUPLICATE_TRIANGLES,
			);
			queue.push(move |world: &mut World| {
				world.entity_mut(entity).insert(c).remove::<ColliderTask>();
			});

			return queue;
		});
		chunk.insert(ColliderTask { task });
		chunks.chunks[index] = chunk.id();
	}
	queue.queue.clear();
}

fn collider_task_resolver(mut chunks: Query<&mut ColliderTask, With<PhosChunk>>, mut commands: Commands) {
	for mut task in &mut chunks {
		if let Some(mut c) = block_on(future::poll_once(&mut task.task)) {
			commands.append(&mut c);
		}
	}
}

#[derive(Component)]
struct ColliderTask {
	pub task: Task<CommandQueue>,
}
