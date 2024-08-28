use bevy::prelude::*;
use shared::states::GameplayState;
use world_generation::states::GeneratorState;

pub struct UnitsDebugPlugin;

impl Plugin for UnitsDebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			spawn_test_unit.run_if(in_state(GeneratorState::Idle).and_then(in_state(GameplayState::Playing))),
		);
	}
}

fn spawn_test_unit(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
	
}
