use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::{systems::InputEvents, EguiContexts};
use bevy_inspector_egui::egui;
use bevy_rapier3d::prelude::*;
use shared::states::GameplayState;
use shared::tags::MainCamera;
use units::units_debug_plugin::UnitsDebugPlugin;
use world_generation::{
	consts::HEX_CORNERS,
	hex_utils::{HexCoord, INNER_RADIUS},
	prelude::Map,
	states::GeneratorState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(UnitsDebugPlugin);
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

		app.add_systems(Update, regenerate_map.run_if(in_state(GeneratorState::Idle)));

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

fn regenerate_map(
	mut generator_state: ResMut<NextState<GeneratorState>>,
	mut gameplay_state: ResMut<NextState<GameplayState>>,
	input: Res<ButtonInput<KeyCode>>,
) {
	if input.just_pressed(KeyCode::KeyR) {
		generator_state.set(GeneratorState::Regenerate);
		gameplay_state.set(GameplayState::PlaceHQ);
	}
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
		if !map.is_in_bounds(&contact_coord) {
			return;
		}
		let height = map.sample_height(&contact_coord);
		gizmos.primitive_3d(
			&shape.0,
			contact_coord.to_world(height + 0.01),
			Quat::IDENTITY,
			Color::WHITE,
		);
		let nbors = map.get_neighbors(&contact_coord);
		let contact_tile_pos = contact_coord.to_world(map.sample_height(&contact_coord));

		for i in 0..6 {
			if let Some(s) = nbors[i] {
				let coord = contact_coord.get_neighbor(i);
				let p = coord.to_world(s);
				gizmos.arrow(p, p + Vec3::Y * (i as f32 + 1.0), Color::WHITE);
			}

			let p = HEX_CORNERS[i] + contact_tile_pos;
			gizmos.arrow(p, p + Vec3::Y * (i as f32 + 1.0), LinearRgba::rgb(1.0, 0.0, 0.5));
		}

		gizmos.line(contact_point, contact_point + Vec3::X, LinearRgba::RED);
		gizmos.line(contact_point, contact_point + Vec3::Y, LinearRgba::GREEN);
		gizmos.line(contact_point, contact_point + Vec3::Z, LinearRgba::BLUE);
		//gizmos.sphere(contact_point, Quat::IDENTITY, 0.1, LinearRgba::rgb(1., 0., 0.5));
	}
}

fn verbose_data() {}
