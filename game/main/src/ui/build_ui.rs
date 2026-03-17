use bevy::{
	camera::{visibility::RenderLayers, CameraOutputMode},
	prelude::*,
	render::render_resource::BlendState,
};
use shared::states::AssetLoadState;

use crate::ui::states::BuildUIState;
pub struct BuildUIPlugin;
#[derive(Component)]
pub struct BuildUIItem;

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
		app.add_systems(PostUpdate, cleanup_ui.run_if(in_state(BuildUIState::Cleanup)));
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
			Name::new("Build UI Root"),
			BuildUIItem,
		))
		// .insert(PickingBehavior::IGNORE)
		.with_children(|build_root| {
			build_root
				.spawn((
					Name::new("Build UI"),
					Node {
						width: Val::Px(500.),
						height: Val::Px(100.),
						justify_content: JustifyContent::Stretch,
						..default()
					},
					BackgroundColor(LinearRgba::GREEN.into()),
				))
				.with_children(|build_ui| {
					build_ui
						.spawn((
							Name::new("Toolbar"),
							Node {
								width: Val::Percent(100.),
								height: Val::Px(30.),
								column_gap: Val::Px(5.),
								padding: UiRect::horizontal(Val::Px(10.)),
								justify_content: JustifyContent::Stretch,
								align_self: AlignSelf::End,
								..default()
							},
							BackgroundColor(LinearRgba::BLUE.into()),
						))
						.with_children(|toolbar| {
							for i in 0..6
							{
								toolbar.spawn((
									Name::new(format!("Button {}", i)),
									Button,
									Node {
										height: Val::Percent(100.),
										width: Val::Percent(100.),
										align_items: AlignItems::Center,
										justify_content: JustifyContent::Center,
										..default()
									},
									BackgroundColor(LinearRgba::WHITE.into()),
									children![(
										Text::new(format!("Button {}", i)),
										TextFont {
											font_size: 15.,
											..default()
										},
										TextColor(LinearRgba::BLACK.into()),
										TextShadow {
											offset: Vec2::splat(2.),
											..default()
										}
									)],
								));
							}
						});
				});
		});

	next_state.set(BuildUIState::Update);
}

fn cleanup_ui(mut commands: Commands, ui_items: Query<Entity, With<BuildUIItem>>)
{
	for item in ui_items.iter()
	{
		commands.entity(item).despawn();
	}
}
