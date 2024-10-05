use std::f32::consts::E;

use bevy::prelude::*;
use shared::{resources::TileUnderCursor, sets::GameplaySet, states::AssetLoadState};
use world_generation::{heightmap, prelude::Map};

use crate::components::{LandUnit, Target, Unit};

pub struct UnitsDebugPlugin;

impl Plugin for UnitsDebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, init.run_if(in_state(AssetLoadState::Loading)));

		app.add_systems(Update, (spawn_test_unit, set_unit_target).in_set(GameplaySet));
	}
}

#[derive(Resource)]
struct TestUnit(pub Handle<Mesh>);

fn init(mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
	let mesh_handle = meshes.add(Cuboid::from_length(1.0));
	commands.insert_resource(TestUnit(mesh_handle));
}

fn spawn_test_unit(
	mut commands: Commands,
	input: Res<ButtonInput<KeyCode>>,
	tile_under_cursor: Res<TileUnderCursor>,
	unit: Res<TestUnit>,
) {
	if !input.just_pressed(KeyCode::KeyT) {
		return;
	}
	if let Some(contact) = tile_under_cursor.0 {
		info!("Spawning Test Unit");
		commands.spawn((
			PbrBundle {
				transform: Transform::from_translation(contact.surface),
				mesh: unit.0.clone(),
				..default()
			},
			Unit,
			LandUnit,
		));
	}
}

fn set_unit_target(
	mut commands: Commands,
	units: Query<Entity, With<Unit>>,
	input: Res<ButtonInput<MouseButton>>,
	tile_under_cursor: Res<TileUnderCursor>,
) {
	if !input.just_pressed(MouseButton::Right) {
		return;
	}
	if let Some(contact) = tile_under_cursor.0 {
		for e in units.iter() {
			info!("Setting Target");
			let mut e = commands.entity(e);
			e.insert(Target(contact.tile));
		}
	}
}
