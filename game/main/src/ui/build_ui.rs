use bevy::{
	prelude::*,
	render::{camera::RenderTarget, view::RenderLayers},
};
use shared::{states::AssetLoadState, tags::MainCamera};
pub struct BuildUIPlugin;

impl Plugin for BuildUIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_cameras);
		app.add_systems(Update, spawn_ui.run_if(in_state(AssetLoadState::LoadComplete)));
	}
}

fn setup_cameras(mut commands: Commands) {
	commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));
}

fn spawn_ui(mut commands: Commands) {
	commands
		.spawn((Node {
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::End,
			..default()
		},))
		.insert(PickingBehavior::IGNORE)
		.with_children(|parent| {
			parent.spawn((
				Node {
					width: Val::Px(500.),
					..Default::default()
				},
				BackgroundColor(LinearRgba::GREEN.into()),
			));
		});
}
