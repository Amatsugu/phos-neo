#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{asset::LoadState, pbr::ExtendedMaterial, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::states::{GameState, GameplayState};
use world_generation::{
	biome_painter::*,
	heightmap::generate_heightmap,
	hex_utils::{offset_to_index, SHORT_DIAGONAL},
	prelude::*,
	tile_manager::*,
	tile_mapper::*,
};

use crate::{
	camera_system::components::*,
	prelude::{ChunkAtlas, PhosChunk, PhosChunkRegistry},
	shader_extensions::chunk_material::ChunkMaterial,
	utlis::{
		chunk_utils::{paint_map, prepare_chunk_mesh_with_collider},
		render_distance_system::RenderDistanceVisibility,
	},
};

use super::{
	chunk_rebuild::ChunkRebuildPlugin, prelude::CurrentBiomePainter, terraforming_test::TerraFormingTestPlugin,
};

pub struct MapInitPlugin;

impl Plugin for MapInitPlugin {
	fn build(&self, app: &mut App) {
		app.insert_state(GeneratorState::GenerateHeightmap);
		app.insert_state(AssetLoadState::StartLoading);

		app.add_plugins((
			ResourceInspectorPlugin::<GenerationConfig>::default(),
			ChunkRebuildPlugin,
			TerraFormingTestPlugin,
		));

		app.add_systems(Startup, (load_textures, load_tiles).in_set(AssetLoaderSet));
		app.add_systems(Startup, create_heightmap.in_set(GeneratorSet));

		app.configure_sets(Startup, AssetLoaderSet.run_if(in_state(AssetLoadState::StartLoading)));
		app.configure_sets(
			Startup,
			GeneratorSet.run_if(in_state(GeneratorState::GenerateHeightmap)),
		);

		app.add_systems(Update, check_asset_load.run_if(in_state(AssetLoadState::Loading)));
		app.add_systems(
			Update,
			finalize_texture.run_if(in_state(AssetLoadState::FinalizeAssets)),
		);
		app.add_systems(Update, despawn_map.run_if(in_state(GeneratorState::Regenerate)));
		app.add_systems(
			Update,
			spawn_map
				.run_if(in_state(AssetLoadState::LoadComplete))
				.run_if(in_state(GeneratorState::SpawnMap)),
		);

		app.insert_resource(TileManager::default());
	}
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GeneratorSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct AssetLoaderSet;

fn load_textures(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
	let main_tex = asset_server.load("textures/world/stack.png");

	let water_material = standard_materials.add(StandardMaterial {
		base_color: Color::AQUAMARINE.with_a(0.5),
		alpha_mode: AlphaMode::Blend,
		..default()
	});
	commands.insert_resource(ChunkAtlas {
		handle: main_tex.clone(),
		is_loaded: false,
		chunk_material_handle: Handle::default(),
		water_material: water_material,
	});
}

fn load_tiles(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut next_state: ResMut<NextState<AssetLoadState>>,
) {
	let handle: Handle<BiomePainterAsset> = asset_server.load("biome_painters/terra.biomes.json");
	commands.insert_resource(CurrentBiomePainter { handle });
	next_state.set(AssetLoadState::Loading);
}

fn check_asset_load(
	asset_server: Res<AssetServer>,
	atlas: Res<ChunkAtlas>,
	painter: Res<CurrentBiomePainter>,
	painter_load: Res<BiomePainterLoadState>,
	tile_load: Res<TileAssetLoadState>,
	mapper_load: Res<TileMapperLoadState>,
	mut next_state: ResMut<NextState<AssetLoadState>>,
) {
	if !painter_load.is_all_loaded() || !tile_load.is_all_loaded() || !mapper_load.is_all_loaded() {
		return;
	}

	if asset_server.load_state(atlas.handle.clone()) != LoadState::Loaded {
		return;
	}
	if asset_server.load_state(painter.handle.clone()) != LoadState::Loaded {
		return;
	}

	next_state.set(AssetLoadState::FinalizeAssets);
}

fn finalize_texture(
	mut atlas: ResMut<ChunkAtlas>,
	mut images: ResMut<Assets<Image>>,
	mut chunk_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ChunkMaterial>>>,
	mut next_state: ResMut<NextState<AssetLoadState>>,
) {
	let image = images.get_mut(&atlas.handle).unwrap();

	let array_layers = 14;
	image.reinterpret_stacked_2d_as_array(array_layers);

	atlas.is_loaded = true;
	let chunk_material = chunk_materials.add(ExtendedMaterial {
		base: StandardMaterial::default(),
		extension: ChunkMaterial {
			array_texture: atlas.handle.clone(),
		},
	});
	atlas.chunk_material_handle = chunk_material;

	next_state.set(AssetLoadState::LoadComplete);
}

fn create_heightmap(
	mut commands: Commands,
	mut cam: Query<(&mut Transform, Entity), With<PhosCamera>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
) {
	let config = GenerationConfig {
		layers: vec![
			GeneratorLayer {
				base_roughness: 2.14,
				roughness: 0.87,
				strength: 8.3,
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
				strength: 3.1,
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
			GeneratorLayer {
				base_roughness: 3.87,
				roughness: 5.8,
				strength: -1.5,
				min_value: 0.,
				persistence: 0.3,
				is_rigid: true,
				weight: 1.,
				weight_multi: 4.57,
				layers: 3,
				first_layer_mask: false,
			},
		],
		noise_scale: 450.,
		sea_level: 8.5,
		border_size: 64.,
		size: UVec2::splat(1024 / Chunk::SIZE as u32),
		// size: UVec2::splat(1),
	};
	let heightmap = generate_heightmap(&config, 4);

	let (mut cam_t, cam_entity) = cam.single_mut();
	cam_t.translation = heightmap.get_center();

	commands.entity(cam_entity).insert(CameraBounds::from_size(config.size));

	commands.insert_resource(heightmap);
	commands.insert_resource(config);
	next_state.set(GeneratorState::SpawnMap);
}

fn spawn_map(
	mut heightmap: ResMut<Map>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	atlas: Res<ChunkAtlas>,
	tile_assets: Res<Assets<TileAsset>>,
	tile_mappers: Res<Assets<TileMapperAsset>>,
	biome_painters: Res<Assets<BiomePainterAsset>>,
	painter: Res<CurrentBiomePainter>,
	mut generator_state: ResMut<NextState<GeneratorState>>,
	cur_game_state: Res<State<GameState>>,
	mut game_state: ResMut<NextState<GameState>>,
	mut gameplay_state: ResMut<NextState<GameplayState>>,
) {
	let b_painter = biome_painters.get(painter.handle.clone());
	let cur_painter = b_painter.unwrap();
	paint_map(&mut heightmap, cur_painter, &tile_assets, &tile_mappers);

	let chunk_meshes: Vec<_> = heightmap
		.chunks
		.par_iter()
		.map(|chunk: &Chunk| {
			let index = offset_to_index(chunk.chunk_offset, heightmap.width);
			return prepare_chunk_mesh_with_collider(&heightmap.get_chunk_mesh_data(index), chunk.chunk_offset, index);
		})
		.collect();

	let mut registry = PhosChunkRegistry::new(chunk_meshes.len());
	{
		#[cfg(feature = "tracing")]
		let _spawn_span = info_span!("Spawn Chunks").entered();
		let visibility_offset = Vec3::new(
			(Chunk::SIZE / 2) as f32 * SHORT_DIAGONAL,
			0.,
			(Chunk::SIZE / 2) as f32 * 1.5,
		);
		for (mesh, collider, pos, index) in chunk_meshes {
			// let mesh_handle = meshes.a
			let chunk = commands.spawn((
				MaterialMeshBundle {
					mesh: meshes.add(mesh),
					material: atlas.chunk_material_handle.clone(),
					transform: Transform::from_translation(pos),
					..default()
				},
				PhosChunk::new(index),
				RenderDistanceVisibility::default().with_offset(visibility_offset),
				collider,
			));
			registry.chunks.push(chunk.id());
		}
	}

	commands.spawn((PbrBundle {
		transform: Transform::from_translation(heightmap.get_center()),
		mesh: meshes.add(
			Plane3d::default()
				.mesh()
				.size(heightmap.get_world_width(), heightmap.get_world_height()),
		),
		material: atlas.water_material.clone(),
		..default()
	},));

	commands.insert_resource(registry);
	generator_state.set(GeneratorState::Idle);
	if cur_game_state.get() != &GameState::Playing {
		game_state.set(GameState::Playing);
		gameplay_state.set(GameplayState::PlaceHQ);
	}
}

fn despawn_map(
	mut commands: Commands,
	mut heightmap: ResMut<Map>,
	cfg: Res<GenerationConfig>,
	chunks: Query<Entity, With<PhosChunk>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
) {
	for chunk in chunks.iter() {
		commands.entity(chunk).despawn();
	}

	*heightmap = generate_heightmap(&cfg, 4);
	next_state.set(GeneratorState::SpawnMap);
}
