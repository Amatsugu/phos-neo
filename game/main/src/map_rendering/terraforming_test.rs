use avian3d::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashSet, window::PrimaryWindow};
use shared::{
	events::{ChunkModifiedEvent, TileModifiedEvent},
	resources::TileUnderCursor,
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
	mut commands: Commands,
	mouse: Res<ButtonInput<MouseButton>>,
	spatial_query: SpatialQuery,
	mut heightmap: ResMut<Map>,
	chunks: Res<PhosChunkRegistry>,
	tile_under_cursor: Res<TileUnderCursor>,
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

	if let Some(contact) = tile_under_cursor.0 {
		#[cfg(feature = "tracing")]
		let span = info_span!("Deform Mesh").entered();
		let modified_tiles = heightmap.create_crater(&contact.tile, 5, 5. * multi);
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
