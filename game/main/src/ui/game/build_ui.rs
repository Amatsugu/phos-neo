use bevy::prelude::*;

pub struct BuildUiPlugin;

impl Plugin for BuildUiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostStartup, setup);
	}
}

fn setup(mut commands: Commands) {}
