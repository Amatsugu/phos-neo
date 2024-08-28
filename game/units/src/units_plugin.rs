use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset_loader::loading_state::{
	config::{ConfigureLoadingState, LoadingStateConfig},
	LoadingStateAppExt,
};
use shared::states::{AssetLoadState, GameplayState};
use world_generation::states::GeneratorState;

use crate::assets::{unit_asset::UnitAssetPlugin, unit_database::UnitDatabase};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(UnitAssetPlugin);

		app.configure_loading_state(LoadingStateConfig::new(AssetLoadState::Loading).load_collection::<UnitDatabase>());

		app.add_systems(Update, units_control.in_set(UnitUpdateSet));

		app.configure_sets(
			Update,
			UnitUpdateSet.run_if(in_state(GameplayState::Playing).and_then(in_state(GeneratorState::Idle))),
		);
	}
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UnitUpdateSet;

fn units_control(input: Res<ButtonInput<KeyCode>>, window: Query<&Window, With<PrimaryWindow>>) {
	let win = window.single();

	let Some(cursor_pos) = win.cursor_position() else {
		return;
	};
}
