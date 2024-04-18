use crate::prelude::*;
use bevy::asset::LoadState;
use bevy::pbr::ExtendedMaterial;
use bevy::{pbr::CascadeShadowConfig, prelude::*};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use camera_system::PhosCameraPlugin;
use iyes_perf_ui::prelude::*;
use world_generation::hex_utils::offset_to_world;
use world_generation::tile_manager::{self, TileAsset, TileAssetPlugin, TileManager};
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
		app.add_systems(Update, (check_texture, spawn_map, print_tiles));

		//Perf UI
		app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
			.add_plugins(PerfUiPlugin);

		//Assets
		app.add_plugins(TileAssetPlugin);
		//Physics
		app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
			.add_plugins(RapierDebugRenderPlugin::default());
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
		transform: Transform::from_xyz(500., 260.0, 500.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	commands.insert_resource(PhosMap::default());
	commands.insert_resource(TileManager::default());
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
	let main_tex = asset_server.load("textures/world/stack.png");
	commands.insert_resource(ChunkAtlas {
		handle: main_tex.clone(),
		is_loaded: false,
	});

	// commands.insert_resource(TileResource(handle));
}

fn load_tiles(
	mut commands: Commands,
	mut tile_manager: ResMut<TileManager>,
	asset_server: Res<AssetServer>,
) {
	let handle: Handle<TileAsset> = asset_server.load("tiles/Terra/Grass.tile.json");
	tile_manager.register_tile(handle);
}

fn print_tiles(tile_assets: Res<Assets<TileAsset>>) {
	for (_, tile) in tile_assets.iter() {
		println!("Tile: {}", tile.name);
	}
}

fn check_texture(
	asset_server: Res<AssetServer>,
	mut atlas: ResMut<ChunkAtlas>,
	mut map: ResMut<PhosMap>,
	mut images: ResMut<Assets<Image>>,
) {
	if atlas.is_loaded {
		return;
	}

	if asset_server.load_state(atlas.handle.clone()) != LoadState::Loaded {
		return;
	}
	let image = images.get_mut(&atlas.handle).unwrap();

	let array_layers = 7;
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
) {
	if !map.ready || !map.regenerate {
		return;
	}
	map.regenerate = false;
	let chunk_material = materials.add(ExtendedMaterial {
		base: StandardMaterial {
			base_color: Color::WHITE,
			..default()
		},
		extension: ChunkMaterial {
			array_texture: atlas.handle.clone(),
		},
	});

	for chunk in &heightmap.chunks {
		let mesh = generate_chunk_mesh(&chunk, &heightmap);
		let pos = offset_to_world(chunk.chunk_offset * Chunk::SIZE as i32, 0.);
		commands.spawn((
			MaterialMeshBundle {
				mesh: meshes.add(mesh),
				material: chunk_material.clone(),
				transform: Transform::from_translation(pos),
				..default()
			},
			PhosChunk,
		));
	}
}
