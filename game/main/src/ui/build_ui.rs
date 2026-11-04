use bevy::{
	camera::{CameraOutputMode, visibility::RenderLayers},
	input_focus::InputFocus,
	prelude::*,
	render::render_resource::BlendState,
};
use buildings::assets::{building_asset::BuildingAsset, building_database::BuildingDatabase};
use shared::{resources::TileUnderCursor, states::AssetLoadState};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tile_viz::components::HexAreaSelection;
use world_generation::states::GeneratorState;

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
				spawn_ui.run_if(in_state(AssetLoadState::LoadComplete).and_then(in_state(BuildUIState::Init))),
				draw_menu_ui.run_if(in_state(AssetLoadState::LoadComplete).and_then(in_state(BuildUIState::DrawMenu))),
			),
		);
		app.init_resource::<SelectionEntity>();

		app.add_systems(OnEnter(BuildUIState::Update), || info!("ui update"));
		app.add_systems(Update, selection_test.run_if(in_state(BuildUIState::Update)));
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

const ITEM_SIZE: f32 = 50.0;
const TOOLBAR_SIZE: f32 = 20.0;

fn spawn_ui(mut commands: Commands, mut next_state: ResMut<NextState<BuildUIState>>)
{
	commands
		.spawn((
			Node {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::End,
				bottom: Val::Px(5.0),
				..default()
			},
			RenderLayers::layer(1),
			Name::new("Build UI Root"),
			BuildUIItem,
			// Interaction::None,
		))
		// .insert(PickingBehavior::IGNORE)
		.with_children(|build_root| {
			build_root
				.spawn((
					Name::new("Build UI"),
					Node {
						width: Val::Px(500.),
						height: Val::Auto,
						justify_content: JustifyContent::Stretch,
						flex_direction: FlexDirection::Column,
						bottom: Val::Px(5.0),
						..default()
					},
					Interaction::None,
				))
				.with_children(|build_ui| {
					build_ui.spawn((
						Name::new("Menu Root"),
						BuildMenuRoot,
						Node {
							width: Val::Percent(100.),
							height: Val::Auto,
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
								height: Val::Px(TOOLBAR_SIZE),
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
										padding: UiRect::horizontal(Val::Px(5.0)),
										..default()
									},
									BaseColor(LinearRgba::WHITE.into()),
									HoverColor(Color::linear_rgb(1.0, 1.0, 0.0)),
									PressedColor(Color::linear_rgb(1.0, 0.5, 1.0)),
									children![(
										Text::new(format!("{:?}", btn)),
										TextFont {
											font_size: FontSize::Px(15.),
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
	building_db: Res<BuildingDatabase>,
	buildings: Res<Assets<BuildingAsset>>,
)
{
	for entity in menu_items.iter() {
		commands.entity(entity).despawn();
	}
	info!("Draw Menu");
	match menu_info.menu {
		BuildUIMenu::Structure => draw_structure_ui(commands, menu_root.into_inner(), building_db, buildings),
		BuildUIMenu::Defense => (),
		BuildUIMenu::Energy => (),
		BuildUIMenu::Production => (),
		BuildUIMenu::Tech => (),
	}
	next.set(BuildUIState::Update);
}

fn draw_structure_ui(
	mut commands: Commands,
	root: Entity,
	building_db: Res<BuildingDatabase>,
	buildings: Res<Assets<BuildingAsset>>,
)
{
	commands.entity(root).with_children(|root| {
		for handle in &building_db.buildings {
			if let Some(building_asset) = buildings.get(handle.id()) {
				root.spawn((
					BuildUIMenuItem,
					Node {
						height: Val::Percent(100.),
						width: Val::Px(ITEM_SIZE),
						display: Display::Grid,
						grid_template_rows: vec![RepeatedGridTrack::px(1, ITEM_SIZE), RepeatedGridTrack::auto(1)],
						..default()
					},
					children![
						(
							Node {
								height: Val::Px(ITEM_SIZE),
								width: Val::Px(ITEM_SIZE),
								..default()
							},
							Button,
							BaseColor(LinearRgba::RED.into()),
							HoverColor(LinearRgba::BLUE.into()),
						),
						(
							Node {
								height: Val::Auto,
								..default()
							},
							Text::new(building_asset.name.clone()),
							TextFont {
								font_size: FontSize::Px(13.),
								..default()
							},
							BaseTextColor(LinearRgba::BLACK.into()),
							HoverTextColor(Color::linear_rgb(0.0, 1.0, 0.0)),
						),
					],
				));
			};
		}
	});
}

fn cleanup_ui(mut commands: Commands, ui_items: Query<Entity, With<BuildUIItem>>)
{
	for entity in ui_items.iter() {
		commands.entity(entity).despawn();
	}
}

#[derive(Resource, Default)]
struct SelectionEntity(pub Option<Entity>);

fn selection_test(
	mut commands: Commands,
	mut selection: ResMut<SelectionEntity>,
	cursor: Res<TileUnderCursor>,
	area: Query<&HexAreaSelection>,
)
{
	if let Some(tile) = cursor.0 {
		if let Some(e) = selection.0
			&& let Ok(area) = area.get(e)
		{
			if area.0 != tile.tile {
				commands.entity(e).insert(HexAreaSelection(tile.tile, 4));
			}
		} else {
			let sel = commands.spawn(HexAreaSelection(tile.tile, 4)).id();
			selection.0 = Some(sel);
		}
	}
}
