#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{pbr::ExtendedMaterial, prelude::*};
use bevy_asset_loader::prelude::*;

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use hex::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::states::{AssetLoadState, GameplayState, MenuState};

use world_generation::{
	biome_asset::{BiomeAsset, BiomeAssetPlugin},
	biome_painter::*,
	heightmap::generate_heightmap,
	mapping::biome_map::BiomeMap,
	prelude::*,
	tile_manager::*,
	tile_mapper::*,
};

use crate::{
	map_rendering::prefabs::ChunkPrefab,
	prelude::{MapRoot, PhosAssets, PhosChunkRegistry, WaterMesh},
	shader_extensions::{
		chunk_material::ChunkMaterial,
		water_material::{WaterMaterial, WaterSettings},
	},
	utils::chunk_utils::{paint_map, prepare_chunk_mesh_with_collider},
};

use super::chunk_rebuild::ChunkRebuildPlugin;

pub struct MapInitPlugin;

impl Plugin for MapInitPlugin
{
	fn build(&self, app: &mut App)
	{
		app.insert_state(GeneratorState::Startup);
		app.insert_state(AssetLoadState::Loading);

		//Assets
		app.add_plugins(TileAssetPlugin);
		app.add_plugins(TileMapperAssetPlugin);
		app.add_plugins(BiomeAssetPlugin);

		app.add_plugins(ResourceInspectorPlugin::<GenerationConfig>::default());
		app.register_type::<ExtendedMaterial<StandardMaterial, WaterMaterial>>();
		app.register_asset_reflect::<ExtendedMaterial<StandardMaterial, WaterMaterial>>();
		app.add_plugins((
			ChunkRebuildPlugin,
			// TerraFormingTestPlugin,
			MaterialPlugin::<ExtendedMaterial<StandardMaterial, ChunkMaterial>>::default(),
			MaterialPlugin::<ExtendedMaterial<StandardMaterial, WaterMaterial>>::default(),
		));

		app.configure_loading_state(
			LoadingStateConfig::new(AssetLoadState::Loading)
				.with_dynamic_assets_file::<StandardDynamicAssetCollection>("phos.assets.ron")
				.load_collection::<PhosAssets>()
				.load_collection::<BiomePainterAsset>(),
		);

		app.add_systems(
			Update,
			create_heightmap.run_if(in_state(GeneratorState::GenerateHeightmap)),
		);

		app.add_systems(
			Update,
			(finalize_texture, setup_materials, finalize_biome_painter)
				.run_if(in_state(AssetLoadState::FinalizeAssets)),
		);

		app.add_systems(Update, despawn_map.run_if(in_state(GeneratorState::Regenerate)));
		app.add_systems(
			Update,
			(spawn_map, configure_water_material)
				.chain()
				.run_if(in_state(AssetLoadState::LoadComplete).and_then(in_state(GeneratorState::SpawnMap))),
		);

		app.insert_resource(TileManager::default());
	}
}

fn setup_materials(
	mut phos_assets: ResMut<PhosAssets>,
	mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterMaterial>>>,
)
{
	let water_material = water_materials.add(ExtendedMaterial {
		base: StandardMaterial {
			base_color: Color::srgb(0., 0.878, 1.),
			alpha_mode: AlphaMode::Blend,
			metallic: 1.0,
			..Default::default()
		},
		extension: WaterMaterial {
			settings: WaterSettings {
				scale: 10.,
				f_power: 0.2,
				deep_color: LinearRgba::rgb(0.0, 0.04, 0.085),
				..Default::default()
			},
		},
	});
	phos_assets.water_material = water_material;
}

fn finalize_biome_painter(
	mut commands: Commands,
	mut next_generator_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainterAsset>,
	biomes: Res<Assets<BiomeAsset>>,
)
{
	let painter = biome_painter.build(&biomes);
	commands.insert_resource(painter);
	next_generator_state.set(GeneratorState::GenerateHeightmap);
}

fn finalize_texture(
	mut atlas: ResMut<PhosAssets>,
	mut images: ResMut<Assets<Image>>,
	mut chunk_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ChunkMaterial>>>,
	mut next_load_state: ResMut<NextState<AssetLoadState>>,
)
{
	let mut image = images.get_mut(atlas.handle.id()).unwrap();

	let array_layers = image.height() / image.width();
	image
		.reinterpret_stacked_2d_as_array(array_layers)
		.expect("Failed to reinterpret as array");

	let chunk_material = chunk_materials.add(ExtendedMaterial {
		base: StandardMaterial::default(),
		extension: ChunkMaterial {
			array_texture: atlas.handle.clone(),
		},
	});
	atlas.chunk_material_handle = chunk_material;

	next_load_state.set(AssetLoadState::LoadComplete);
}

fn create_heightmap(
	mut commands: Commands,
	mut next_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainter>,
)
{
	info!("Generate Heightmap");
	let config = GenerationConfig {
		biome_blend: 32,
		biome_dither: 10.,
		continent_noise: NoiseConfig {
			scale: 800.,
			layers: vec![GeneratorLayer {
				base_roughness: 2.14,
				roughness: 0.87,
				strength: 100.,
				min_value: 0.,
				persistence: 0.77,
				is_rigid: false,
				weight: 0.,
				weight_multi: 0.,
				layers: 1,
			}],
		},
		moisture_noise: NoiseConfig {
			scale: 900.,
			layers: vec![GeneratorLayer {
				base_roughness: 2.14,
				roughness: 0.87,
				strength: 100.,
				min_value: 0.,
				persistence: 0.77,
				is_rigid: false,
				weight: 0.,
				weight_multi: 0.,
				layers: 1,
			}],
		},
		temperature_noise: NoiseConfig {
			scale: 700.,
			layers: vec![GeneratorLayer {
				base_roughness: 2.14,
				roughness: 0.87,
				strength: 100.,
				min_value: 0.,
				persistence: 0.77,
				is_rigid: false,
				weight: 0.,
				weight_multi: 0.,
				layers: 1,
			}],
		},
		sea_level: 8.5,
		border_size: 64.,
		size: UVec2::new(16, 16),
		// size: UVec2::splat(1),
	};
	let (heightmap, biome_map) = generate_heightmap(&config, 42069, &biome_painter);

	commands.insert_resource(heightmap);
	commands.insert_resource(biome_map);
	commands.insert_resource(config);
	next_state.set(GeneratorState::SpawnMap);
}

fn configure_water_material(
	mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterMaterial>>>,
	atlas: Res<PhosAssets>,
	heightmap: Res<Map>,
)
{
	info!("Update sealevel");
	if let Some(mut material) = water_materials.get_mut(atlas.water_material.id()) {
		material.extension.settings.surface_level = heightmap.sealevel;
	}
}

fn spawn_map(
	mut heightmap: ResMut<Map>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	atlas: Res<PhosAssets>,
	tile_assets: Res<Assets<TileAsset>>,
	tile_mappers: Res<Assets<TileMapperAsset>>,
	mut generator_state: ResMut<NextState<GeneratorState>>,
	cur_game_state: Res<State<MenuState>>,
	mut game_state: ResMut<NextState<MenuState>>,
	mut gameplay_state: ResMut<NextState<GameplayState>>,
	biome_painter: Res<BiomePainter>,
)
{
	info!("Spawn Map");
	paint_map(&mut heightmap, &biome_painter, &tile_assets, &tile_mappers);

	let root = commands.spawn(MapRoot).id();
	//Prepare Mesh Data
	let map_size = UVec2::new(heightmap.width as u32, heightmap.height as u32);
	let chunk_meshes: Vec<_> = heightmap
		.chunks
		.par_iter()
		.map(|chunk: &Chunk| {
			let index = offset_to_index(chunk.chunk_offset, heightmap.width);
			let world_pos = chunk.world_position();
			let (chunk_mesh, water_mesh, collider) =
				prepare_chunk_mesh_with_collider(&heightmap.get_chunk_mesh_data(index), heightmap.sealevel, map_size);
			return (chunk_mesh, water_mesh, collider, world_pos, index);
		})
		.collect();

	let mut registry = PhosChunkRegistry::new(chunk_meshes.len());

	//Spawn Chunks
	{
		#[cfg(feature = "tracing")]
		let _spawn_span = info_span!("Spawn Chunks").entered();

		for (chunk_mesh, water_mesh, collider, pos, index) in chunk_meshes {
			let chunk_handle = meshes.add(chunk_mesh);
			let chunk = commands
				.spawn(ChunkPrefab::terrain(
					pos,
					chunk_handle,
					atlas.chunk_material_handle.clone(),
					collider,
					index,
				))
				.id();
			if let Some(water_mesh) = water_mesh {
				let water_mesh_handle = meshes.add(water_mesh);
				let water = commands
					.spawn(ChunkPrefab::water(
						Vec3::ZERO,
						water_mesh_handle.clone(),
						atlas.water_material.clone(),
						index,
					))
					.id();
				commands
					.entity(chunk)
					.insert(WaterMesh(water_mesh_handle.id(), water))
					.add_child(water);
				registry.waters.push(Some(water));
			} else {
				registry.waters.push(None);
			}
			commands.entity(root).add_child(chunk);
			registry.chunks.push(chunk);
		}
	}

	commands.insert_resource(registry);

	generator_state.set(GeneratorState::Idle);
	info!("Generator Idle");

	if cur_game_state.get() != &MenuState::InGame {
		game_state.set(MenuState::InGame);
		gameplay_state.set(GameplayState::PlaceHQ);
	}
}

fn despawn_map(
	mut commands: Commands,
	mut heightmap: ResMut<Map>,
	mut biome_map: ResMut<BiomeMap>,
	cfg: Res<GenerationConfig>,
	map_root: Single<Entity, With<MapRoot>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainter>,
)
{
	info!("Despawn Map");
	commands.entity(map_root.into_inner()).despawn();

	(*heightmap, *biome_map) = generate_heightmap(&cfg, 4, &biome_painter);
	next_state.set(GeneratorState::SpawnMap);
}
