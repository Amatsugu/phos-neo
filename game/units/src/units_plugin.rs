use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::loading_state::{
	config::{ConfigureLoadingState, LoadingStateConfig},
	LoadingStateAppExt,
};
use shared::{sets::GameplaySet, states::AssetLoadState};
use world_generation::{hex_utils::HexCoord, prelude::Map};

use crate::{
	assets::{unit_asset::UnitAssetPlugin, unit_database::UnitDatabase},
	components::{Target, Unit},
	units_debug_plugin::UnitsDebugPlugin,
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(UnitAssetPlugin);

		#[cfg(debug_assertions)]
		app.add_plugins(UnitsDebugPlugin);

		// app.configure_loading_state(LoadingStateConfig::new(AssetLoadState::Loading).load_collection::<UnitDatabase>());

		app.add_systems(Update, units_control.in_set(GameplaySet));
		app.add_systems(Update, move_unit.in_set(GameplaySet));
	}
}

fn units_control(input: Res<ButtonInput<KeyCode>>, window: Query<&Window, With<PrimaryWindow>>) {
	let win = window.single();

	let Some(cursor_pos) = win.cursor_position() else {
		return;
	};
}

fn move_unit(mut units: Query<(&mut Transform, &Target), With<Unit>>, time: Res<Time>, map: Res<Map>) {
	for (mut t, target) in units.iter_mut() {
		let vel = (target.0 - t.translation).normalize() * 10.0 * time.delta_seconds();
		t.translation += vel;
		let coord = HexCoord::from_world_pos(t.translation);
		if map.is_in_bounds(&coord) {
			t.translation.y = map.sample_height(&coord);
		}
	}
}
