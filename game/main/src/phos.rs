use crate::camera_system::camera_plugin::PhosCameraPlugin;
use crate::camera_system::components::PhosCamera;
use crate::map_rendering::chunk_rebuild::ChunkRebuildPlugin;
use crate::map_rendering::map_init::MapInitPlugin;
use crate::shader_extensions::chunk_material::ChunkMaterial;
use crate::utlis::render_distance_system::RenderDistancePlugin;
use bevy::pbr::ExtendedMaterial;
use bevy::{
	pbr::{wireframe::WireframeConfig, CascadeShadowConfig},
	prelude::*,
};
use bevy_rapier3d::dynamics::{Ccd, RigidBody, Velocity};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use buildings::BuildingPugin;
use iyes_perf_ui::prelude::*;
use shared::despawn::DespawnPuglin;
use shared::states::{GameState, GameplayState};
use world_generation::biome_painter::BiomePainterPlugin;
use world_generation::tile_manager::TileAssetPlugin;
use world_generation::tile_mapper::TileMapperAssetPlugin;

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			PhosCameraPlugin,
			MapInitPlugin,
			RenderDistancePlugin,
			BuildingPugin,
			DespawnPuglin,
		));

		app.insert_state(GameState::Startup);
		app.insert_state(GameplayState::Waiting);

		//Systems - Startup
		app.add_systems(Startup, init_game);

		//Systems - Update
		app.add_systems(Update, spawn_sphere);

		//Perf UI
		app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
			.add_plugins(PerfUiPlugin);

		//Assets
		app.add_plugins(TileAssetPlugin);
		app.add_plugins(TileMapperAssetPlugin);
		app.add_plugins(BiomePainterPlugin);
		//Physics
		app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
		// app.add_plugins(RapierDebugRenderPlugin::default());

		app.insert_resource(WireframeConfig {
			global: false,
			default_color: Color::hex("FF0064").unwrap(),
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
		base_color: Color::CYAN,
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
			Collider::ball(0.3),
			RigidBody::Dynamic,
			Ccd::enabled(),
			Velocity::linear(cam_transform.forward() * 50.),
		));
	}
}
