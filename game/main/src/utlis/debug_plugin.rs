use bevy::prelude::*;
use shared::resources::TileUnderCursor;
use shared::states::GameplayState;
use world_generation::{consts::HEX_CORNERS, prelude::Map, states::GeneratorState};

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

		gizmos.line(contact.point, contact.point + Vec3::X, LinearRgba::RED);
		gizmos.line(contact.point, contact.point + Vec3::Y, LinearRgba::GREEN);
		gizmos.line(contact.point, contact.point + Vec3::Z, LinearRgba::BLUE);
	}
}

fn verbose_data() {}
