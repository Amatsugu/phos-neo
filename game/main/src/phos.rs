use crate::camera_system::camera_plugin::PhosCameraPlugin;
use crate::camera_system::components::PhosCamera;
use crate::map_rendering::map_init::MapInitPlugin;
use crate::utlis::render_distance_system::RenderDistancePlugin;
use avian3d::prelude::*;
use avian3d::PhysicsPlugins;
use bevy::{
	pbr::{wireframe::WireframeConfig, CascadeShadowConfig},
	prelude::*,
};
use bevy_asset_loader::prelude::*;
use buildings::BuildingPugin;
use iyes_perf_ui::prelude::*;
use shared::states::{GameplayState, MenuState};
use shared::{despawn::DespawnPuglin, states::AssetLoadState};

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
	fn build(&self, app: &mut App) {
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
			//BuildingPugin,
			DespawnPuglin,
		));

		//Systems - Startup
		app.add_systems(Startup, init_game);

		//Systems - Update
		app.add_systems(Update, spawn_sphere);

		//Perf UI
		app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
			.add_plugins(PerfUiPlugin);

		//Physics
		app.add_plugins(PhysicsPlugins::default());
		// app.add_plugins(RapierDebugRenderPlugin::default());

		app.insert_resource(WireframeConfig {
			global: false,
			default_color: Srgba::hex("FF0064").unwrap().into(),
		});
	}
}

fn init_game(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
	commands.spawn((
		PerfUiRoot::default(),
		PerfUiEntryFPS::default(),
		PerfUiEntryFPSWorst::default(),
		PerfUiEntryFrameTime::default(),
		PerfUiEntryFrameTimeWorst::default(),
	));

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			..default()
		},
		cascade_shadow_config: CascadeShadowConfig {
			bounds: vec![200., 400., 600., 800.],
			..default()
		},
		transform: Transform::from_xyz(500., 260.0, 500.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

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
	cam: Query<&Transform, With<PhosCamera>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mat: Res<SphereMat>,
) {
	if keyboard_input.just_pressed(KeyCode::KeyF) {
		let cam_transform = cam.single();
		commands.spawn((
			MaterialMeshBundle {
				mesh: meshes.add(Sphere::new(0.3)),
				material: mat.0.clone(),
				transform: Transform::from_translation(cam_transform.translation),
				..default()
			},
			Collider::sphere(0.3),
			RigidBody::Dynamic,
			LinearVelocity(cam_transform.forward() * 50.),
		));
	}
}
