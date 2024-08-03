use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;
use bevy_rapier3d::prelude::*;
use shared::tags::MainCamera;
use world_generation::{
	consts::HEX_CORNERS,
	hex_utils::{HexCoord, INNER_RADIUS},
	prelude::Map,
	states::GeneratorState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.insert_state(DebugState::Base);

		app.add_systems(
			Update,
			show_tile_heights
				.run_if(in_state(GeneratorState::Idle))
				.run_if(not(in_state(DebugState::None))),
		);

		app.add_systems(
			Update,
			verbose_data
				.run_if(in_state(GeneratorState::Idle))
				.run_if(in_state(DebugState::Verbose)),
		);

		app.insert_resource(Shape(Polyline3d::new([
			HEX_CORNERS[0],
			HEX_CORNERS[1],
			HEX_CORNERS[2],
			HEX_CORNERS[3],
			HEX_CORNERS[4],
			HEX_CORNERS[5],
			HEX_CORNERS[0],
		])));
	}
}

#[derive(Resource)]
struct Shape(pub Polyline3d<7>);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugState {
	Base,
	None,
	Verbose,
}

fn show_tile_heights(
	cam_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
	window: Query<&Window, With<PrimaryWindow>>,
	rapier_context: Res<RapierContext>,
	map: Res<Map>,
	mut gizmos: Gizmos,
	shape: Res<Shape>,
) {
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

	if let Some((_e, dist)) = collision {
		let contact_point = cam_ray.get_point(dist);
		let contact_coord = HexCoord::from_world_pos(contact_point);
		if map.is_in_bounds(&contact_coord) {
			let height = map.sample_height(&contact_coord);
			gizmos.primitive_3d(
				&shape.0,
				contact_coord.to_world(height + 0.01),
				Quat::IDENTITY,
				Color::WHITE,
			);
		}
		gizmos.sphere(contact_point, Quat::IDENTITY, 0.1, Srgba::rgb(1., 0., 0.5));
	}
}

fn verbose_data() {}
