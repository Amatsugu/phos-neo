use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::{prelude::*, window::PrimaryWindow};
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
	spatial_query: SpatialQuery,
	map: Res<Map>,
	mut tile_under_cursor: ResMut<TileUnderCursor>,
) {
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
		500.,
		true,
		SpatialQueryFilter::default(),
	);

	if let Some(data) = collision {
		let dist = data.time_of_impact;
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
