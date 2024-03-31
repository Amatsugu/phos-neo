use bevy::asset::io::memory::Value::Vec;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{pbr::CascadeShadowConfig, prelude::*};
use camera_system::PhosCameraPlugin;
use iyes_perf_ui::prelude::*;
use world_generation::hex_utils::{offset_to_world, HexCoord};
use world_generation::{
	heightmap::generate_heightmap, hex_utils::offset3d_to_world,
	mesh_generator::generate_chunk_mesh, prelude::*,
};

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PhosCameraPlugin);
		app.add_systems(Startup, init_game)
			.add_systems(Startup, create_map)
			.add_systems(Update, draw_gizmos);
		app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
			.add_plugins(PerfUiPlugin);
		app.add_plugins(WireframePlugin);
		// app.insert_resource(WireframeConfig {
		// 	global: true,
		// 	default_color: Color::CYAN,
		// });
	}
}

fn init_game(mut commands: Commands) {
	commands.spawn((
		PerfUiRoot::default(),
		PerfUiEntryFPS::default(),
		PerfUiEntryClock::default(),
	));

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			..default()
		},
		cascade_shadow_config: CascadeShadowConfig {
			bounds: vec![500., 1000., 5000., 10000.],
			..default()
		},
		transform: Transform::from_xyz(500., 160.0, 500.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

fn draw_gizmos(mut gizmos: Gizmos, hm: Res<Map>) {
	gizmos.arrow(Vec3::ZERO, Vec3::Y * 1.5, Color::GREEN);
	gizmos.arrow(Vec3::ZERO, Vec3::Z * 1.5, Color::BLUE);
	gizmos.arrow(Vec3::ZERO, Vec3::X * 1.5, Color::RED);

	let ch = &hm.chunks[0];
	let coord = HexCoord::new(16, 16);
	let h = ch.points[coord.to_chunk_local_index() as usize];
	gizmos.ray(coord.to_world(h), Vec3::Y, Color::RED);
	gizmos.ray(coord.to_world(h), Vec3::Z * 1.5, Color::BLUE);

	// let t = coord.get_neighbor(5);
	// let h = ch.points[t.to_chunk_local_index() as usize];
	// gizmos.ray(t.to_world(h), Vec3::Y * 1., Color::PINK);
	let n = coord.get_neighbors();
	for i in 0..6 {
		let t = n[i];
		let h = ch.points[t.to_chunk_local_index() as usize];
		gizmos.ray(t.to_world(h), Vec3::Y * (i + 1) as f32, Color::CYAN);
	}
}

fn create_map(
	mut commands: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let heightmap = generate_heightmap(
		1,
		1,
		&GenerationConfig {
			layers: vec![
				GeneratorLayer {
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
				},
				GeneratorLayer {
					base_roughness: 2.85,
					roughness: 2.,
					strength: -0.23,
					min_value: -0.,
					persistence: 1.,
					is_rigid: false,
					weight: 0.,
					weight_multi: 0.,
					layers: 4,
					first_layer_mask: false,
				},
				GeneratorLayer {
					base_roughness: 2.6,
					roughness: 4.,
					strength: 10.44,
					min_value: 0.,
					persistence: 1.57,
					is_rigid: true,
					weight: 1.,
					weight_multi: 0.35,
					layers: 4,
					first_layer_mask: true,
				},
				GeneratorLayer {
					base_roughness: 3.87,
					roughness: 5.8,
					strength: -1.,
					min_value: 0.,
					persistence: 0.,
					is_rigid: true,
					weight: 1.,
					weight_multi: 4.57,
					layers: 3,
					first_layer_mask: true,
				},
			],
			noise_scale: 350.,
			sea_level: 4.,
		},
		2,
	);

	let debug_material = materials.add(StandardMaterial {
		// base_color_texture: Some(images.add(uv_debug_texture())),
		..default()
	});

	for chunk in &heightmap.chunks {
		let mesh = generate_chunk_mesh(&chunk, &heightmap);
		let pos = offset_to_world(chunk.chunk_offset * Chunk::SIZE as i32, 0.);
		commands.spawn(PbrBundle {
			mesh: meshes.add(mesh),
			material: debug_material.clone(),
			transform: Transform::from_translation(pos),
			..default()
		});
	}

	commands.insert_resource(heightmap);
}
