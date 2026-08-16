use avian3d::{
	collision::collider::{Collider, CollisionMargin},
	dynamics::{ccd::SpeculativeMargin, rigid_body::RigidBody},
};
#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{
	light::NotShadowCaster,
	pbr::ExtendedMaterial,
	prelude::*,
	tasks::{AsyncComputeTaskPool, Task, futures},
};
use bevy_asset_loader::prelude::*;

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use hex::prelude::*;
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
	map_rendering::render_distance_system::RenderDistanceVisibility,
	prelude::{MapRoot, PhosAssets, PhosChunk},
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

		app.add_systems(OnEnter(GeneratorState::GenerateHeightmap), create_heightmap);

		app.add_systems(
			Update,
			resolve_heightmap_task.run_if(in_state(GeneratorState::WaitForHeightmap)),
		);

		app.add_systems(
			OnEnter(AssetLoadState::FinalizeAssets),
			(finalize_texture, setup_materials, finalize_biome_painter),
		);

		app.add_systems(Update, despawn_map.run_if(in_state(GeneratorState::Regenerate)));
		app.add_systems(OnEnter(GeneratorState::BuildChunkMesh), configure_water_material);
		app.add_systems(
			Update,
			start_chunk_build.run_if(in_state(GeneratorState::BuildChunkMesh)),
		);
		app.add_systems(
			Update,
			chunk_build_finalize
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
			double_sided: true,
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
	info!("Start Heightmap Generation");
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
		size: UVec2::splat(12),
		// size: UVec2::splat(1),
	};

	commands.insert_resource(config.clone());
	let config = config.clone();
	let biome_painter = biome_painter.clone();
	let task_pool = AsyncComputeTaskPool::get();
	let task = task_pool.spawn(async move {
		return generate_heightmap(&config, 42069, &biome_painter);
	});

	commands.spawn(HeightmapGenerationTask(task));

	next_state.set(GeneratorState::WaitForHeightmap);
}

fn resolve_heightmap_task(
	heightmap_task: Single<(Entity, &mut HeightmapGenerationTask)>,
	mut commands: Commands,
	mut next_state: ResMut<NextState<GeneratorState>>,
)
{
	let (entity, mut task) = heightmap_task.into_inner();
	if let Some((map, biomes)) = futures::check_ready(&mut task.0) {
		info!("Heightmap Generation Complete");
		commands.insert_resource(map);
		commands.insert_resource(biomes);
		next_state.set(GeneratorState::BuildChunkMesh);
		commands.entity(entity).despawn();
	}
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

fn start_chunk_build(
	mut heightmap: ResMut<Map>,
	mut commands: Commands,
	tile_assets: Res<Assets<TileAsset>>,
	tile_mappers: Res<Assets<TileMapperAsset>>,
	biome_painter: Res<BiomePainter>,
	mut next_state: ResMut<NextState<GeneratorState>>,
)
{
	info!("Start Chunk Build");
	paint_map(&mut heightmap, &biome_painter, &tile_assets, &tile_mappers);
	let root = commands.spawn(MapRoot).id();
	let task_pool = AsyncComputeTaskPool::get();
	let map_size = UVec2::new(heightmap.width as u32, heightmap.height as u32);

	for chunk in &heightmap.chunks {
		let index = offset_to_index(chunk.chunk_offset, heightmap.width);
		let mesh_data = heightmap.get_chunk_mesh_data(index);
		let sealevel = heightmap.sealevel;
		let task = task_pool.spawn(async move { prepare_chunk_mesh_with_collider(&mesh_data, sealevel, map_size) });
		let chunk_entity = commands
			.spawn((
				PhosChunk::new(index),
				Name::new(format!("Chunk {}", index)),
				ChunkBuildTask(task),
				Transform::from_translation(chunk.world_position()),
				InheritedVisibility::VISIBLE,
				SpeculativeMargin(0.5),
				RigidBody::Static,
				CollisionMargin(0.1),
			))
			.id();
		commands.entity(root).add_child(chunk_entity);
	}
	next_state.set(GeneratorState::SpawnMap);
}

fn chunk_build_finalize(
	build_tasks: Query<(Entity, &PhosChunk, &mut ChunkBuildTask)>,
	atlas: Res<PhosAssets>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
	cur_game_state: Res<State<MenuState>>,
	mut game_state: ResMut<NextState<MenuState>>,
	mut gameplay_state: ResMut<NextState<GameplayState>>,
	mut commands: Commands,
)
{
	if build_tasks.is_empty() {
		info!("All chunks spawned");
		next_state.set(GeneratorState::Idle);
		if cur_game_state.get() != &MenuState::InGame {
			game_state.set(MenuState::InGame);
			gameplay_state.set(GameplayState::PlaceHQ);
		}
		return;
	}
	for (entity, chunk, mut task) in build_tasks {
		let Some((terrain_mesh, water_mesh, collider)) = futures::check_ready(&mut task.0) else {
			continue;
		};
		commands.entity(entity).insert((
			Mesh3d(meshes.add(terrain_mesh)),
			MeshMaterial3d(atlas.chunk_material_handle.clone()),
			RenderDistanceVisibility::chunk_centered(),
			collider,
		));
		if let Some(water_mesh) = water_mesh {
			let water_entity = commands
				.spawn((
					Mesh3d(meshes.add(water_mesh)),
					MeshMaterial3d(atlas.water_material.clone()),
					chunk.clone(),
					Name::new(format!("Water {}", chunk.index)),
					NotShadowCaster,
					Transform::from_translation(Vec3::ZERO),
				))
				.id();
			commands.entity(entity).add_child(water_entity);
		}
		commands.entity(entity).remove::<ChunkBuildTask>();
	}
}

fn despawn_map(
	mut commands: Commands,
	cfg: Res<GenerationConfig>,
	map_root: Single<Entity, With<MapRoot>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainter>,
)
{
	info!("Despawn Map");
	commands.entity(map_root.into_inner()).despawn();

	let config = cfg.clone();
	let biome_painter = biome_painter.clone();
	let task_pool = AsyncComputeTaskPool::get();
	let task = task_pool.spawn(async move {
		return generate_heightmap(&config, 42069, &biome_painter);
	});

	commands.spawn(HeightmapGenerationTask(task));
	next_state.set(GeneratorState::WaitForHeightmap);
}

#[derive(Component)]
struct HeightmapGenerationTask(pub Task<(Map, BiomeMap)>);

#[derive(Component)]
struct ChunkBuildTask(pub Task<(Mesh, Option<Mesh>, Collider)>);
