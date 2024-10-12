use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};
use shared::{
	resources::{TileContact, TileUnderCursor},
	tags::MainCamera,
};
use world_generation::{hex_utils::HexCoord, prelude::Map, states::GeneratorState};
pub struct TileSelectionPlugin;

impl Plugin for TileSelectionPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<TileUnderCursor>();
		app.add_systems(
			PreUpdate,
			update_tile_under_cursor.run_if(in_state(GeneratorState::Idle)),
		);
	}
}

fn update_tile_under_cursor(
	cam_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
	window: Query<&Window, With<PrimaryWindow>>,
	rapier_context: Res<RapierContext>,
	map: Res<Map>,
	mut tile_under_cursor: ResMut<TileUnderCursor>,
) {
	let win_r = window.get_single();
	if win_r.is_err() {
		return;
	}
	let win = win_r.unwrap();

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

	if let Some((_e, dist)) = collision {
		let contact_point = cam_ray.get_point(dist);
		let contact_coord = HexCoord::from_world_pos(contact_point);
		//todo: handle correct tile detection when contacting a tile from the side
		if !map.is_in_bounds(&contact_coord) {
			tile_under_cursor.0 = None;
			return;
		}
		let surface = map.sample_height(&contact_coord);
		tile_under_cursor.0 = Some(TileContact::new(
			contact_coord,
			contact_point,
			contact_coord.to_world(surface),
		));
	} else {
		tile_under_cursor.0 = None;
	}
}
