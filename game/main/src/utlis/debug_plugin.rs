use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::{systems::InputEvents, EguiContexts};
use bevy_inspector_egui::egui;
use shared::resources::TileUnderCursor;
use shared::states::GameplayState;
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

fn show_tile_heights(map: Res<Map>, mut gizmos: Gizmos, shape: Res<Shape>, tile_under_cursor: Res<TileUnderCursor>) {
	if let Some(contact) = tile_under_cursor.0 {
		let height = map.sample_height(&contact.tile);
		gizmos.primitive_3d(
			&shape.0,
			contact.tile.to_world(height + 0.01),
			Quat::IDENTITY,
			Color::WHITE,
		);
		let nbors = map.get_neighbors(&contact.tile);
		let contact_tile_pos = contact.tile.to_world(map.sample_height(&contact.tile));

		// for i in 0..6 {
		// 	if let Some(s) = nbors[i] {
		// 		let coord = contact.tile.get_neighbor(i);
		// 		let p = coord.to_world(s);
		// 		gizmos.arrow(p, p + Vec3::Y * (i as f32 + 1.0), Color::WHITE);
		// 	}

		// 	let p = HEX_CORNERS[i] + contact_tile_pos;
		// 	gizmos.arrow(p, p + Vec3::Y * (i as f32 + 1.0), LinearRgba::rgb(1.0, 0.0, 0.5));
		// }

		gizmos.line(contact.point, contact.point + Vec3::X, LinearRgba::RED);
		gizmos.line(contact.point, contact.point + Vec3::Y, LinearRgba::GREEN);
		gizmos.line(contact.point, contact.point + Vec3::Z, LinearRgba::BLUE);
		//gizmos.sphere(contact_point, Quat::IDENTITY, 0.1, LinearRgba::rgb(1., 0., 0.5));
	}
}

fn verbose_data() {}
