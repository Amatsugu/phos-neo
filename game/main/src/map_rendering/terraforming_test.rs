use bevy::prelude::*;
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};
use world_generation::{hex_utils::HexCoord, prelude::Map};

use crate::camera_system::components::PhosCamera;

use super::chunk_rebuild::ChunkRebuildQueue;

pub struct TerraFormingTestPlugin;

impl Plugin for TerraFormingTestPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, deform);
	}
}

fn deform(
	cam: Query<&Transform, With<PhosCamera>>,
	keyboard: Res<ButtonInput<KeyCode>>,
	rapier_context: Res<RapierContext>,
	mut heightmap: ResMut<Map>,
	mut rebuild: ResMut<ChunkRebuildQueue>,
) {
	if !keyboard.pressed(KeyCode::KeyF) {
		return;
	}

	let cam_transform = cam.single();
	let fwd: Vec3 = cam_transform.forward().into();

	let collision = rapier_context.cast_ray(cam_transform.translation, fwd, 100., true, QueryFilter::only_fixed());

	if let Some((entity, dist)) = collision {
		let contact_point = cam_transform.translation + (fwd * dist);
		let contact_coord = HexCoord::from_world_pos(contact_point);
	}
}
