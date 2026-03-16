use bevy::{
	camera::{visibility::RenderLayers, CameraOutputMode, Viewport},
	prelude::*,
	render::render_resource::BlendState,
};
use shared::states::AssetLoadState;

use crate::ui::states::BuildUIState;
pub struct BuildUIPlugin;

impl Plugin for BuildUIPlugin
{
	fn build(&self, app: &mut App)
	{
		app.add_systems(Startup, setup_cameras);
		app.insert_state(BuildUIState::Init);
		app.add_systems(
			Update,
			spawn_ui.run_if(in_state(AssetLoadState::LoadComplete).and(in_state(BuildUIState::Init))),
		);
	}
}

fn setup_cameras(mut commands: Commands)
{
	commands
		.spawn((
			Camera2d,
			Camera {
				order: 1,
				clear_color: ClearColorConfig::None,
				msaa_writeback: MsaaWriteback::Always,
				// viewport: Some(Viewport {
				// 	physical_size: UVec2::new(800, 800),
				// 	..default()
				// }),
				output_mode: CameraOutputMode::Write {
					blend_state: Some(BlendState::ALPHA_BLENDING),
					clear_color: ClearColorConfig::None,
				},
				..default()
			},
			IsDefaultUiCamera,
		))
		.insert(RenderLayers::layer(1))
		.insert(Msaa::Off);
}

fn spawn_ui(mut commands: Commands, mut next_state: ResMut<NextState<BuildUIState>>)
{
	commands
		.spawn((
			Node {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::End,
				..default()
			},
			RenderLayers::layer(1),
		))
		// .insert(PickingBehavior::IGNORE)
		.with_children(|parent| {
			parent.spawn((
				Node {
					width: Val::Px(500.),
					height: Val::Px(100.),
					..Default::default()
				},
				BackgroundColor(LinearRgba::GREEN.into()),
			));
		});

	next_state.set(BuildUIState::Update);
}
