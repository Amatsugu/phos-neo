#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::{
	pbr::{ExtendedMaterial, NotShadowCaster},
	prelude::*,
	render::texture::ImageFormat,
};
use bevy_asset_loader::prelude::*;

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use image::DynamicImage;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::states::{AssetLoadState, GameplayState, MenuState};
use world_generation::{
	biome_asset::{BiomeAsset, BiomeAssetPlugin},
	biome_painter::*,
	heightmap::generate_heightmap,
	hex_utils::{offset_to_index, SHORT_DIAGONAL},
	map::{
		biome_map::{self, BiomeMap},
		map_utils::{render_biome_noise_map, render_map},
	},
	prelude::*,
	tile_manager::*,
	tile_mapper::*,
};

use crate::{
	camera_system::components::*,
	prelude::{PhosAssets, PhosChunk, PhosChunkRegistry},
	shader_extensions::{
		chunk_material::ChunkMaterial,
		water_material::{WaterMaterial, WaterSettings},
	},
	utlis::{
		chunk_utils::{paint_map, prepare_chunk_mesh_with_collider},
		render_distance_system::RenderDistanceVisibility,
	},
};

use super::{chunk_rebuild::ChunkRebuildPlugin, terraforming_test::TerraFormingTestPlugin};

pub struct MapInitPlugin;

impl Plugin for MapInitPlugin {
	fn build(&self, app: &mut App) {
		app.insert_state(GeneratorState::Startup);
		app.insert_state(AssetLoadState::Loading);

		//Assets
		app.add_plugins(TileAssetPlugin);
		app.add_plugins(TileMapperAssetPlugin);
		app.add_plugins(BiomeAssetPlugin);

		app.add_plugins(ResourceInspectorPlugin::<GenerationConfig>::default());
		app.add_plugins(ResourceInspectorPlugin::<WaterInspect>::default());
		app.register_type::<ExtendedMaterial<StandardMaterial, WaterMaterial>>();
		app.register_asset_reflect::<ExtendedMaterial<StandardMaterial, WaterMaterial>>();
		app.add_plugins((
			ChunkRebuildPlugin,
			// TerraFormingTestPlugin,
			MaterialPlugin::<ExtendedMaterial<StandardMaterial, ChunkMaterial>>::default(),
			MaterialPlugin::<ExtendedMaterial<StandardMaterial, WaterMaterial>> {
				prepass_enabled: false,
				..Default::default()
			},
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
			spawn_map.run_if(in_state(AssetLoadState::LoadComplete).and_then(in_state(GeneratorState::SpawnMap))),
		);

		app.insert_resource(TileManager::default());
	}
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct WaterInspect(Handle<ExtendedMaterial<StandardMaterial, WaterMaterial>>);

fn setup_materials(
	mut commands: Commands,
	mut phos_assets: ResMut<PhosAssets>,
	mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterMaterial>>>,
) {
	let water_material = water_materials.add(ExtendedMaterial {
		base: StandardMaterial {
			base_color: Color::srgba(0., 0.5, 1., 0.8),
			alpha_mode: AlphaMode::Blend,
			..Default::default()
		},
		extension: WaterMaterial {
			settings: WaterSettings {
				offset: 0.5,
				scale: 100.,
				..Default::default()
			},
			..default()
		},
	});
	commands.insert_resource(WaterInspect(water_material.clone()));
	phos_assets.water_material = water_material;
}

fn finalize_biome_painter(
	mut commands: Commands,
	mut next_generator_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainterAsset>,
	biomes: Res<Assets<BiomeAsset>>,
) {
	let biome_painter = biome_painter.build(&biomes);
	commands.insert_resource(biome_painter);
	next_generator_state.set(GeneratorState::GenerateHeightmap);
}

fn finalize_texture(
	mut atlas: ResMut<PhosAssets>,
	mut images: ResMut<Assets<Image>>,
	mut chunk_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ChunkMaterial>>>,
	mut next_load_state: ResMut<NextState<AssetLoadState>>,
) {
	let image = images.get_mut(atlas.handle.id()).unwrap();

	let array_layers = image.height() / image.width();
	image.reinterpret_stacked_2d_as_array(array_layers);

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
	mut cam: Query<(&mut Transform, Entity), With<PhosCamera>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainter>,
) {
	let config = GenerationConfig {
		biome_blend: 16,
		biome_dither: 16.,
		continent_noise: NoiseConfig {
			scale: 1000.,
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
				first_layer_mask: false,
			}],
		},
		moisture_noise: NoiseConfig {
			scale: 500.,
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
				first_layer_mask: false,
			}],
		},
		temperature_noise: NoiseConfig {
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
				first_layer_mask: false,
			}],
		},
		sea_level: 8.5,
		border_size: 64.,
		size: UVec2::splat(16),
		// size: UVec2::splat(1),
	};
	let (heightmap, biome_map) = generate_heightmap(&config, 42069, &biome_painter);

	let (mut cam_t, cam_entity) = cam.single_mut();
	cam_t.translation = heightmap.get_center();

	commands.entity(cam_entity).insert(CameraBounds::from_size(config.size));
	commands.insert_resource(heightmap);
	commands.insert_resource(biome_map);
	commands.insert_resource(config);
	next_state.set(GeneratorState::SpawnMap);
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
) {
	paint_map(&mut heightmap, &biome_painter, &tile_assets, &tile_mappers);

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

	commands.spawn((
		MaterialMeshBundle {
			transform: Transform::from_translation(heightmap.get_center()),
			mesh: meshes.add(
				Plane3d::default()
					.mesh()
					.size(heightmap.get_world_width(), heightmap.get_world_height()),
			),
			material: atlas.water_material.clone(),
			..default()
		},
		NotShadowCaster,
	));

	commands.insert_resource(registry);
	generator_state.set(GeneratorState::Idle);
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
	chunks: Query<Entity, With<PhosChunk>>,
	mut next_state: ResMut<NextState<GeneratorState>>,
	biome_painter: Res<BiomePainter>,
) {
	for chunk in chunks.iter() {
		commands.entity(chunk).despawn();
	}

	(*heightmap, *biome_map) = generate_heightmap(&cfg, 4, &biome_painter);
	next_state.set(GeneratorState::SpawnMap);
}
