use bevy::{pbr::CascadeShadowConfig, prelude::*};
use iyes_perf_ui::prelude::*;
use world_generation::{
	heightmap::generate_heightmap, hex_utils::to_hex_pos, mesh_generator::generate_chunk_mesh,
	prelude::*,
};
pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_game)
			.add_systems(Startup, create_map);
		app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
			.add_plugins(PerfUiPlugin);
	}
}

fn init_game(mut commands: Commands) {
	commands.spawn((Camera3dBundle {
		transform: Transform::from_xyz(-200., 300., -200.)
			.looking_at(Vec3::new(1000., 0., 1000.), Vec3::Y),
		..default()
	},));

	commands.spawn((
		PerfUiRoot::default(),
		PerfUiEntryFPS::default(),
		PerfUiEntryClock::default(),
	));

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: false,
			..default()
		},
		cascade_shadow_config: CascadeShadowConfig {
			bounds: vec![20., 40., 80., 1000., 5000., 19000., 20000.],
			..default()
		},
		transform: Transform::from_xyz(0.0, 16.0, 5.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

fn create_map(
	mut commands: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let heightmap = generate_heightmap(
		32,
		32,
		&GenerationConfig {
			layers: vec![GeneratorLayer {
				base_roughness: 2.14,
				roughness: 0.87,
				strength: 2.93,
				min_value: -0.2,
				persistence: 0.77,
				is_rigid: false,
				weight: 0.,
				weight_multi: 0.,
				layers: 4,
				first_layer_mask: false,
			}],
			noise_scale: 250.,
			sea_level: 4.,
		},
		1,
	);

	let debug_material = materials.add(StandardMaterial {
		// base_color_texture: Some(images.add(uv_debug_texture())),
		..default()
	});

	for chunk in heightmap.chunks {
		let mesh = generate_chunk_mesh(&chunk);
		let pos = to_hex_pos(
			Vec3::new(chunk.chunk_offset.x as f32, 0., chunk.chunk_offset.y as f32)
				* chunk.size as f32,
		);
		commands.spawn(PbrBundle {
			mesh: meshes.add(mesh),
			material: debug_material.clone(),
			transform: Transform::from_translation(pos),
			..default()
		});
	}
}
