use crate::camera_system::camera_plugin::PhosCameraPlugin;
use crate::camera_system::components::PhosCamera;
use crate::map_rendering::map_init::MapInitPlugin;
use crate::map_rendering::render_distance_system::RenderDistancePlugin;
#[cfg(feature = "terraforming")]
use crate::map_rendering::terraforming_test::TerraFormingTestPlugin;
use crate::ui::build_ui::BuildUIPlugin;
use crate::ui::ui_base::BaseUIPlugin;
#[cfg(debug_assertions)]
use crate::utils::debug_plugin::DebugPlugin;
use crate::utils::tile_selection_plugin::TileSelectionPlugin;
use avian3d::prelude::*;
use bevy::dev_tools::diagnostics_overlay::{DiagnosticsOverlay, DiagnosticsOverlayPlugin};
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::light::CascadeShadowConfig;
use bevy::{pbr::wireframe::WireframeConfig, prelude::*};
use bevy_asset_loader::prelude::*;
use shared::sets::GameplaySystems;
use shared::states::{GameplayState, MenuState};
use shared::{despawn::DespawnPlugin, states::AssetLoadState};
use world_generation::states::GeneratorState;

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin
{
	fn build(&self, app: &mut App)
	{
		app.insert_state(AssetLoadState::Loading);
		app.insert_state(MenuState::Loading);
		app.insert_state(GameplayState::Waiting);

		app.add_loading_state(
			LoadingState::new(AssetLoadState::Loading).continue_to_state(AssetLoadState::FinalizeAssets),
		);

		app.add_plugins((
			PhosCameraPlugin,
			MapInitPlugin,
			RenderDistancePlugin,
			// BuildingPugin,
			BaseUIPlugin,
			BuildUIPlugin,
			#[cfg(feature = "terraforming")]
			TerraFormingTestPlugin,
			// SimpleAnimationPlugin,
			// UnitsPlugin,
			DespawnPlugin,
			TileSelectionPlugin,
			#[cfg(feature = "editor")]
			crate::utils::editor_plugin::EditorPlugin,
			#[cfg(debug_assertions)]
			DebugPlugin,
		));

		configure_gameplay_set(app);

		//Systems - Startup
		app.add_systems(Startup, init_game);

		//Systems - Update
		app.add_systems(Update, spawn_sphere);

		//Perf UI
		app.add_plugins((
			FrameTimeDiagnosticsPlugin::default(),
			EntityCountDiagnosticsPlugin::default(),
			bevy::diagnostic::SystemInformationDiagnosticsPlugin,
			DiagnosticsOverlayPlugin,
		));
		// .add_plugins(PerfUiPlugin);

		//Physics
		app.add_plugins(PhysicsPlugins::default());
		// app.add_plugins(RapierDebugRenderPlugin::default());

		app.insert_resource(WireframeConfig {
			global: false,
			default_color: Srgba::hex("FF0064").unwrap().into(),
			..default()
		});
	}
}

fn configure_gameplay_set(app: &mut App)
{
	app.configure_sets(
		Update,
		GameplaySystems.run_if(in_state(GeneratorState::Idle).and_then(in_state(MenuState::InGame))),
	);
	app.configure_sets(
		PreUpdate,
		GameplaySystems.run_if(in_state(GeneratorState::Idle).and_then(in_state(MenuState::InGame))),
	);
	app.configure_sets(
		PostUpdate,
		GameplaySystems.run_if(in_state(GeneratorState::Idle).and_then(in_state(MenuState::InGame))),
	);

	app.configure_sets(
		FixedUpdate,
		GameplaySystems.run_if(in_state(GeneratorState::Idle).and_then(in_state(MenuState::InGame))),
	);
	app.configure_sets(
		FixedPreUpdate,
		GameplaySystems.run_if(in_state(GeneratorState::Idle).and_then(in_state(MenuState::InGame))),
	);
	app.configure_sets(
		FixedPostUpdate,
		GameplaySystems.run_if(in_state(GeneratorState::Idle).and_then(in_state(MenuState::InGame))),
	);
}

fn init_game(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>)
{
	commands.spawn(DiagnosticsOverlay::fps());

	commands.spawn((
		DirectionalLight {
			shadow_maps_enabled: true,
			..default()
		},
		CascadeShadowConfig {
			bounds: vec![200., 400., 600., 800.],
			..default()
		},
		Transform::from_xyz(500., 260.0, 500.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let sphere_mat = StandardMaterial {
		base_color: Color::srgb(1., 1., 0.),
		..default()
	};
	let handle = materials.add(sphere_mat);
	commands.insert_resource(SphereMat(handle));
}

#[derive(Resource)]
struct SphereMat(Handle<StandardMaterial>);

fn spawn_sphere(
	mut commands: Commands,
	cam: Single<&Transform, With<PhosCamera>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mat: Res<SphereMat>,
)
{
	if keyboard_input.just_pressed(KeyCode::KeyF) {
		commands.spawn((
			Mesh3d(meshes.add(Sphere::new(0.3))),
			MeshMaterial3d(mat.0.clone()),
			Transform::from_translation(cam.translation),
			Collider::sphere(0.3),
			RigidBody::Dynamic,
			// Ccd::enabled(),
			LinearVelocity(cam.forward() * 50.),
		));
	}
}
