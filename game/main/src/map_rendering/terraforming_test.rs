use bevy::{prelude::*, utils::hashbrown::HashSet, window::PrimaryWindow};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};
use shared::{
	events::{ChunkModifiedEvent, TileModifiedEvent},
	states::GameplayState,
};
use world_generation::{hex_utils::HexCoord, prelude::Map, states::GeneratorState};

use crate::{
	camera_system::components::PhosCamera,
	prelude::{PhosChunkRegistry, RebuildChunk},
};

pub struct TerraFormingTestPlugin;

impl Plugin for TerraFormingTestPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			deform
				.run_if(in_state(GeneratorState::Idle))
				.run_if(in_state(GameplayState::Playing)),
		);
	}
}

fn deform(
	cam_query: Query<(&GlobalTransform, &Camera), With<PhosCamera>>,
	mut commands: Commands,
	window: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<ButtonInput<MouseButton>>,
	rapier_context: Res<RapierContext>,
	mut heightmap: ResMut<Map>,
	chunks: Res<PhosChunkRegistry>,
	mut chunk_modified: EventWriter<ChunkModifiedEvent>,
	mut tile_modified: EventWriter<TileModifiedEvent>,
) {
	let mut multi = 0.;
	if mouse.just_pressed(MouseButton::Left) {
		multi = 1.;
	} else if mouse.just_pressed(MouseButton::Right) {
		multi = -1.;
	}

	if multi == 0. {
		return;
	}

	let win = window.single();
	let (cam_transform, camera) = cam_query.single();
	let Some(cursor_pos) = win.cursor_position() else {
		return;
	};

	let Some(cam_ray) = camera.viewport_to_world(cam_transform, cursor_pos) else {
		return;
	};

	let collision = rapier_context.cast_ray(
		cam_ray.origin,
		cam_ray.direction.into(),
		500.,
		true,
		QueryFilter::only_fixed(),
	);

	if let Some((_, dist)) = collision {
		#[cfg(feature = "tracing")]
		let span = info_span!("Deform Mesh").entered();
		let contact_point = cam_ray.get_point(dist);
		let contact_coord = HexCoord::from_world_pos(contact_point);
		let modified_tiles = heightmap.create_crater(&contact_coord, 5, 5. * multi);
		let mut chunk_set: HashSet<usize> = HashSet::new();
		for (tile, height) in modified_tiles {
			let chunk = tile.to_chunk_index(heightmap.width);
			if !chunk_set.contains(&chunk) {
				chunk_modified.send(ChunkModifiedEvent { index: chunk });
				chunk_set.insert(chunk);
				commands.entity(chunks.chunks[chunk]).insert(RebuildChunk);
			}
			tile_modified.send(TileModifiedEvent::HeightChanged(tile, height));
		}
		// commands.entity(e).insert(RebuildChunk);
	}
}
