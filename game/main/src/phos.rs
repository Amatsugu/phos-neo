use bevy::{pbr::CascadeShadowConfig, prelude::*};
use camera_system::PhosCameraPlugin;
use iyes_perf_ui::prelude::*;
use world_generation::hex_utils::{offset_to_world, HexCoord};
use world_generation::{
	heightmap::generate_heightmap, mesh_generator::generate_chunk_mesh, prelude::*,
};

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PhosCameraPlugin);
		app.add_systems(Startup, init_game)
			.add_systems(Startup, create_map);
		app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
			.add_plugins(PerfUiPlugin);
	}
}

fn init_game(mut commands: Commands) {
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
			bounds: vec![500., 1000., 2000., 5000.],
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

	let coord = HexCoord::from_grid_pos(64, 14);
	let ch = &hm.chunks[coord.to_chunk_index(hm.width) as usize];
	let h = ch.points[coord.to_chunk_local_index() as usize];
	gizmos.ray(coord.to_world(h), Vec3::Y, Color::RED);
	gizmos.ray(coord.to_world(h), Vec3::Z * 1.5, Color::BLUE);

	// let t = coord.get_neighbor(5);
	// let h = ch.points[t.to_chunk_local_index() as usize];
	// gizmos.ray(t.to_world(h), Vec3::Y * 1., Color::PINK);
	let n = coord.get_neighbors();
	let nh = hm.get_neighbors(&coord);
	for i in 0..6 {
		let t = n[i];
		let h = nh[i];
		if h.is_none() {
			continue;
		}
		gizmos.ray(
			t.to_world(h.unwrap()),
			Vec3::Y * (i + 1) as f32,
			Color::CYAN,
		);
	}
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
