use crate::prelude::*;
use crate::shader_extensions::chunk_material::ChunkMaterial;
use bevy::asset::LoadState;
use bevy::pbr::{ExtendedMaterial, PbrPlugin};
use bevy::{
	pbr::{wireframe::WireframeConfig, CascadeShadowConfig},
	prelude::*,
};
use bevy_rapier3d::dynamics::{RigidBody, Velocity};
use bevy_rapier3d::geometry::{Collider, Restitution};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use camera_system::prelude::PhosCamera;
use camera_system::PhosCameraPlugin;
use iyes_perf_ui::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use world_generation::biome_painter::{
	BiomePainterAsset, BiomePainterLoadState, BiomePainterPlugin,
};
use world_generation::hex_utils::offset_to_world;
use world_generation::mesh_generator::generate_chunk_collider;
use world_generation::tile_manager::{TileAsset, TileAssetLoadState, TileAssetPlugin, TileManager};
use world_generation::tile_mapper::{TileMapperAsset, TileMapperAssetPlugin, TileMapperLoadState};
use world_generation::{
	heightmap::generate_heightmap, mesh_generator::generate_chunk_mesh, prelude::*,
};

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PhosCameraPlugin)
			.add_plugins(MaterialPlugin::<
				ExtendedMaterial<StandardMaterial, ChunkMaterial>,
			>::default());

		//Systems - Startup
		app.add_systems(Startup, init_game)
			.add_systems(Startup, (load_textures, load_tiles, create_map).chain());

		//Systems - Update
		app.add_systems(Update, (finalize_texture, spawn_map, spawn_sphere));

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
		// .add_plugins(RapierDebugRenderPlugin::default());

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
			bounds: vec![500., 1000., 1500., 2000.],
			..default()
		},
		transform: Transform::from_xyz(500., 260.0, 500.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	commands.insert_resource(PhosMap::default());
	commands.insert_resource(TileManager::default());

	let sphere_mat = StandardMaterial {
		base_color: Color::CYAN,
		..default()
	};
	let handle = materials.add(sphere_mat);
	commands.insert_resource(SphereMat(handle));
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
	let main_tex = asset_server.load("textures/world/stack.png");
	commands.insert_resource(ChunkAtlas {
		handle: main_tex.clone(),
		is_loaded: false,
	});
}
#[derive(Resource)]
struct Painter(Handle<BiomePainterAsset>);

fn load_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
	let handle: Handle<BiomePainterAsset> = asset_server.load("biome_painters/terra.biomes.json");
	commands.insert_resource(Painter(handle));
}

fn finalize_texture(
	asset_server: Res<AssetServer>,
	mut atlas: ResMut<ChunkAtlas>,
	mut map: ResMut<PhosMap>,
	mut images: ResMut<Assets<Image>>,
	painter: Res<Painter>,
	painter_load: Res<BiomePainterLoadState>,
	tile_load: Res<TileAssetLoadState>,
	mapper_load: Res<TileMapperLoadState>,
) {
	if atlas.is_loaded {
		return;
	}

	if !painter_load.is_all_loaded() || !tile_load.is_all_loaded() || !mapper_load.is_all_loaded() {
		return;
	}

	if asset_server.load_state(atlas.handle.clone()) != LoadState::Loaded {
		return;
	}
	if asset_server.load_state(painter.0.clone()) != LoadState::Loaded {
		return;
	}
	let image = images.get_mut(&atlas.handle).unwrap();

	let array_layers = 14;
	image.reinterpret_stacked_2d_as_array(array_layers);

	atlas.is_loaded = true;
	map.ready = true;
	map.regenerate = true;
}

fn create_map(mut commands: Commands) {
	let heightmap = generate_heightmap(
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
			border_size: 64.,
			size: UVec2::splat(1024 / Chunk::SIZE as u32),
		},
		4,
	);

	commands.insert_resource(heightmap);
}

fn spawn_map(
	heightmap: Res<Map>,
	mut commands: Commands,
	mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ChunkMaterial>>>,
	mut meshes: ResMut<Assets<Mesh>>,
	atlas: Res<ChunkAtlas>,
	mut map: ResMut<PhosMap>,
	tile_assets: Res<Assets<TileAsset>>,
	tile_mappers: Res<Assets<TileMapperAsset>>,
	biome_painters: Res<Assets<BiomePainterAsset>>,
	painter: Res<Painter>,
) {
	if !map.ready || !map.regenerate {
		return;
	}
	let b_painter = biome_painters.get(painter.0.clone());
	map.regenerate = false;
	let chunk_material = materials.add(ExtendedMaterial {
		base: StandardMaterial::default(),
		extension: ChunkMaterial {
			array_texture: atlas.handle.clone(),
		},
	});

	let cur_painter = b_painter.unwrap();

	let chunk_meshes: Vec<_> = heightmap
		.chunks
		.par_iter()
		.map(|chunk: &Chunk| {
			let mesh =
				generate_chunk_mesh(chunk, &heightmap, cur_painter, &tile_assets, &tile_mappers);
			let collision = generate_chunk_collider(chunk, &heightmap);
			return (
				mesh,
				collision,
				offset_to_world(chunk.chunk_offset * Chunk::SIZE as i32, 0.),
			);
		})
		.collect();

	for (mesh, (col_verts, col_indicies), pos) in chunk_meshes {
		commands.spawn((
			MaterialMeshBundle {
				mesh: meshes.add(mesh),
				material: chunk_material.clone(),
				transform: Transform::from_translation(pos),
				..default()
			},
			PhosChunk,
			Collider::trimesh(col_verts, col_indicies),
		));
	}
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
				mesh: meshes.add(Sphere::new(0.5)),
				material: mat.0.clone(),
				transform: Transform::from_translation(cam_transform.translation),
				..default()
			},
			Collider::ball(0.5),
			RigidBody::Dynamic,
			Velocity::linear(cam_transform.forward() * 100.),
		));
	}
}
// fn render_distance_system(
// 	mut chunks: Query<(&PhosChunk, &mut Visibility)>,
// 	camera: Query<&Transform, With<PhosCamera>>,
// ) {
// 	let cam = camera.single();
// 	for (transform, mut visibility) in chunks.iter_mut() {
// 		let dist = (transform.center - cam.translation).length();
// 		if dist > 500. {
// 			*visibility = Visibility::Hidden;
// 		} else {
// 			*visibility = Visibility::Visible;
// 		}
// 	}
// }
