use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::*;
use bevy::utils::futures;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::geometry::TriMeshFlags;
use world_generation::prelude::Map;

use crate::prelude::RebuildChunk;
use crate::{
	prelude::{PhosChunk, PhosChunkRegistry},
	utlis::chunk_utils::prepare_chunk_mesh,
};

pub struct ChunkRebuildPlugin;

impl Plugin for ChunkRebuildPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ChunkRebuildQueue::default());
		app.init_resource::<PhosChunkRegistry>();
		app.add_systems(PreUpdate, chunk_rebuilder);
		app.add_systems(PostUpdate, collider_task_resolver);
	}
}

#[derive(Resource, Default)]
pub struct ChunkRebuildQueue {
	pub queue: Vec<usize>,
}

fn chunk_rebuilder(
	mut commands: Commands,
	chunk_query: Query<(Entity, &PhosChunk), With<RebuildChunk>>,
	heightmap: Res<Map>,
) {
	let pool = AsyncComputeTaskPool::get();

	for (chunk_entity, idx) in &chunk_query {
		#[cfg(feature = "tracing")]
		let _spawn_span = info_span!("Rebuild Chunk").entered();

		let chunk_index = idx.index;
		let chunk_data = heightmap.get_chunk_mesh_data(chunk_index);
		let chunk_offset = heightmap.chunks[chunk_index].chunk_offset;

		let task = pool.spawn(async move {
			#[cfg(feature = "tracing")]
			let _spawn_span = info_span!("Rebuild Task").entered();
			let mut queue = CommandQueue::default();
			let (mesh, collider_data, _, _) = prepare_chunk_mesh(&chunk_data, chunk_offset, chunk_index);
			#[cfg(feature = "tracing")]
			let trimesh_span = info_span!("Chunk Trimesh").entered();
			let c = Collider::trimesh_with_flags(
				collider_data.0,
				collider_data.1,
				TriMeshFlags::DELETE_DUPLICATE_TRIANGLES,
			);
			#[cfg(feature = "tracing")]
			drop(trimesh_span);
			queue.push(move |world: &mut World| {
				world.entity_mut(chunk_entity).insert(c).remove::<ChunkRebuildTask>();
			});

			return (queue, mesh);
		});

		commands
			.entity(chunk_entity)
			.insert(ChunkRebuildTask { task })
			.remove::<RebuildChunk>();
	}
}

fn collider_task_resolver(
	mut chunks: Query<(&mut ChunkRebuildTask, &Handle<Mesh>), With<PhosChunk>>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	for (mut task, mesh_handle) in &mut chunks {
		if let Some((mut c, mesh)) = futures::check_ready(&mut task.task) {
			commands.append(&mut c);
			meshes.insert(mesh_handle, mesh);
		}
	}
}

#[derive(Component)]
struct ChunkRebuildTask {
	pub task: Task<(CommandQueue, Mesh)>,
}
