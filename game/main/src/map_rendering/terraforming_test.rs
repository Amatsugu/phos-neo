use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_3d::plugins::spatial_query::{SpatialQuery, SpatialQueryFilter};
use world_generation::{hex_utils::HexCoord, prelude::Map};

use crate::{camera_system::components::PhosCamera, prelude::PhosChunkRegistry};

use super::chunk_rebuild::ChunkRebuildQueue;

pub struct TerraFormingTestPlugin;

impl Plugin for TerraFormingTestPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, deform);
	}
}

fn deform(
	cam_query: Query<(&GlobalTransform, &Camera), With<PhosCamera>>,
	window: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<ButtonInput<MouseButton>>,
	spatial_query: SpatialQuery,
	mut heightmap: ResMut<Map>,
	mut rebuild: ResMut<ChunkRebuildQueue>,
	time: Res<Time>,
) {
	let mut multi = 0.;
	if mouse.pressed(MouseButton::Left) {
		multi = 1.;
	} else if mouse.pressed(MouseButton::Right) {
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

	let collision = spatial_query.cast_ray(
		cam_ray.origin,
		cam_ray.direction.into(),
		100.,
		true,
		SpatialQueryFilter::default(),
	);

	if let Some(hit) = collision {
		let contact_point = cam_ray.get_point(hit.time_of_impact);
		let contact_coord = HexCoord::from_world_pos(contact_point);
		let cur_height = heightmap.sample_height(&contact_coord);
		heightmap.set_height(&contact_coord, cur_height + 1. * time.delta_seconds() * multi);
		let cur_chunk = contact_coord.to_chunk_index(heightmap.width);

		if contact_coord.is_on_chunk_edge() {
			let neighbors = contact_coord.get_neighbors();
			let mut other_chunks: Vec<_> = neighbors
				.iter()
				.map(|c| c.to_chunk_index(heightmap.width))
				.filter(|c| c != &cur_chunk)
				.collect();
			rebuild.queue.append(&mut other_chunks);
		}
		rebuild.queue.push(cur_chunk);
	}
}
