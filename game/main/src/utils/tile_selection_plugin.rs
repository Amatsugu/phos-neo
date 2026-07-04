use avian3d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};
use hex::prelude::*;
use shared::{
	resources::{TileContact, TileUnderCursor},
	tags::MainCamera,
};
use world_generation::{prelude::Map, states::GeneratorState};

pub struct TileSelectionPlugin;

impl Plugin for TileSelectionPlugin
{
	fn build(&self, app: &mut App)
	{
		app.init_resource::<TileUnderCursor>();
		app.add_systems(
			PreUpdate,
			setup_cursor_castor.run_if(in_state(GeneratorState::SpawnMap)),
		);
		app.add_systems(
			PreUpdate,
			update_tile_under_cursor.run_if(in_state(GeneratorState::Idle)),
		);

		app.add_systems(PostUpdate, update_caster.run_if(in_state(GeneratorState::Idle)));
	}
}

#[derive(Component)]
#[require(RayCaster)]
struct CursorCaster;

fn setup_cursor_castor(mut commands: Commands)
{
	commands.spawn((CursorCaster, RayCaster::default().with_max_hits(1)));
}

fn update_caster(
	mut cursor: Single<&mut RayCaster>,
	window: Single<&Window, With<PrimaryWindow>>,
	cam_query: Single<(&GlobalTransform, &Camera), With<MainCamera>>,
)
{
	let (cam_transform, camera) = cam_query.into_inner();
	let Some(cursor_pos) = window.cursor_position() else {
		return;
	};

	let Ok(cam_ray) = camera.viewport_to_world(cam_transform, cursor_pos) else {
		return;
	};

	cursor.direction = cam_ray.direction;
	cursor.origin = cam_ray.origin;
}

fn update_tile_under_cursor(
	cursor: Single<(&RayCaster, &RayHits), With<CursorCaster>>,
	map: Res<Map>,
	mut tile_under_cursor: ResMut<TileUnderCursor>,
)
{
	let (ray, hits) = cursor.into_inner();
	if let Some(hit) = hits.first() {
		let contact_point = ray.get_global_point(hit.distance);
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
