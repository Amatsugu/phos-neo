use bevy::{
	camera::{visibility::RenderLayers, CameraOutputMode},
	input_focus::InputFocus,
	prelude::*,
	render::render_resource::BlendState,
};
use shared::states::AssetLoadState;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::ui::{components::*, events::Press, states::BuildUIState};
pub struct BuildUIPlugin;

#[derive(Component, Default)]
struct BuildUIItem;

#[derive(Component, Default)]
#[require(BuildUIItem)]
struct BuildUIMenuItem;

#[derive(Component, Default)]
struct BuildMenuRoot;

#[derive(Resource, Reflect, Debug, Default)]
struct BuildUIInfo
{
	menu: BuildUIMenu,
}

#[derive(Debug, Reflect, Component)]
struct ToolbarButton(pub BuildUIMenu);

#[derive(Default, Reflect, Debug, EnumIter, Clone, Copy)]
enum BuildUIMenu
{
	#[default]
	Structure,
	Defense,
	Energy,
	Production,
	Tech,
}

impl Plugin for BuildUIPlugin
{
	fn build(&self, app: &mut App)
	{
		app.add_systems(Startup, setup_cameras);
		app.init_resource::<BuildUIInfo>().init_resource::<InputFocus>();
		app.insert_state(BuildUIState::Init);
		app.add_observer(toolbar_click);
		app.add_systems(
			Update,
			(
				spawn_ui.run_if(in_state(AssetLoadState::LoadComplete).and(in_state(BuildUIState::Init))),
				draw_menu_ui.run_if(in_state(AssetLoadState::LoadComplete).and(in_state(BuildUIState::DrawMenu))),
			),
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
						flex_direction: FlexDirection::Column,
						..default()
					},
					BackgroundColor(LinearRgba::GREEN.into()),
				))
				.with_children(|build_ui| {
					build_ui.spawn((
						Name::new("Menu Root"),
						BuildMenuRoot,
						Node {
							width: Val::Percent(100.),
							height: Val::Px(72.),
							column_gap: Val::Px(5.),
							padding: UiRect::all(Val::Px(2.)),
							flex_direction: FlexDirection::Row,
							overflow: Overflow::scroll_x(),
							..default()
						},
					));
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
							for btn in BuildUIMenu::iter() {
								toolbar.spawn((
									Name::new(format!("{:?}", btn)),
									Button,
									ToolbarButton(btn),
									Node {
										height: Val::Percent(100.),
										width: Val::Percent(100.),
										align_items: AlignItems::Center,
										justify_content: JustifyContent::Center,
										..default()
									},
									BaseColor(LinearRgba::WHITE.into()),
									HoverColor(Color::linear_rgb(1.0, 1.0, 0.0)),
									PressedColor(Color::linear_rgb(1.0, 0.5, 1.0)),
									children![(
										Text::new(format!("{:?}", btn)),
										TextFont {
											font_size: 15.,
											..default()
										},
										BaseTextColor(LinearRgba::BLACK.into()),
										HoverTextColor(Color::linear_rgb(0.0, 1.0, 0.0)),
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

	next_state.set(BuildUIState::DrawMenu);
}

fn toolbar_click(
	click: On<Press>,
	button: Query<&ToolbarButton>,
	mut menu_info: ResMut<BuildUIInfo>,
	mut next: ResMut<NextState<BuildUIState>>,
)
{
	if let Ok(btn) = button.get(click.0) {
		menu_info.menu = btn.0;
		next.set(BuildUIState::DrawMenu);
	}
}

fn draw_menu_ui(
	mut commands: Commands,
	menu_root: Single<Entity, With<BuildMenuRoot>>,
	menu_items: Query<Entity, With<BuildUIMenuItem>>,
	menu_info: Res<BuildUIInfo>,
	mut next: ResMut<NextState<BuildUIState>>,
)
{
	for entity in menu_items.iter() {
		commands.entity(entity).despawn();
	}
	info!("Draw Menu");
	match menu_info.menu {
		BuildUIMenu::Structure => draw_structure_ui(commands, menu_root.into_inner()),
		BuildUIMenu::Defense => (),
		BuildUIMenu::Energy => (),
		BuildUIMenu::Production => (),
		BuildUIMenu::Tech => (),
	}
	next.set(BuildUIState::Update);
}

fn draw_structure_ui(mut commands: Commands, root: Entity)
{
	commands.entity(root).with_children(|root| {
		for i in 0..10 {
			root.spawn((
				BuildUIMenuItem,
				Node {
					height: Val::Percent(100.),
					width: Val::Px(70.),
					display: Display::Grid,
					grid_template_rows: vec![RepeatedGridTrack::px(1, 100.), RepeatedGridTrack::fr(1, 1.)],
					..default()
				},
				children![
					(
						Node {
							height: Val::Px(70.),
							width: Val::Px(70.),
							..default()
						},
						Button,
						BaseColor(LinearRgba::RED.into()),
						HoverColor(LinearRgba::BLUE.into())
					),
					(Text::new(format!("Icon {}", i))),
				],
			));
		}
	});
}

fn cleanup_ui(mut commands: Commands, ui_items: Query<Entity, With<BuildUIItem>>)
{
	for entity in ui_items.iter() {
		commands.entity(entity).despawn();
	}
}
