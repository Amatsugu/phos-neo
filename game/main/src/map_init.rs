use bevy::{asset::LoadState, pbr::ExtendedMaterial, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_rapier3d::geometry::{Collider, TriMeshFlags};
use camera_system::prelude::PhosCamera;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use world_generation::{
	biome_painter::*,
	chunk_colliders::generate_chunk_collider,
	heightmap::generate_heightmap,
	hex_utils::{offset_to_world, tile_to_world_distance},
	mesh_generator::generate_chunk_mesh,
	prelude::*,
	tile_manager::*,
	tile_mapper::*,
};

use crate::{
	prelude::{ChunkAtlas, PhosChunk, PhosMap},
	shader_extensions::chunk_material::ChunkMaterial,
	utlis::render_distance_system::RenderDistanceVisibility,
};

pub struct MapInitPlugin;

impl Plugin for MapInitPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			ResourceInspectorPlugin::<PhosMap>::default(),
			ResourceInspectorPlugin::<GenerationConfig>::default(),
		));

		app.add_systems(Startup, (load_textures, load_tiles, create_map));

		app.add_systems(Update, finalize_texture);
		app.add_systems(PostUpdate, (despawn_map, spawn_map).chain());
		app.insert_resource(TileManager::default());
		app.insert_resource(PhosMap::default());
	}
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

fn create_map(mut commands: Commands, mut cam: Query<&mut Transform, With<PhosCamera>>) {
	let config = GenerationConfig {
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
				strength: 4.3,
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
				first_layer_mask: true,
			},
		],
		noise_scale: 450.,
		sea_level: 8.5,
		border_size: 64.,
		size: UVec2::splat(1024 / Chunk::SIZE as u32),
		// size: UVec2::splat(1),
	};
	let heightmap = generate_heightmap(&config, 4);

	commands.insert_resource(heightmap);

	// let mut cam_t = cam.single_mut();
	// cam_t.translation = Vec3::new(
	// 	tile_to_world_distance(config.size.x as i32 / 2),
	// 	cam_t.translation.y,
	// 	tile_to_world_distance(config.size.y as i32 / 2),
	// );

	commands.insert_resource(config);
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
			let mesh = generate_chunk_mesh(chunk, &heightmap, cur_painter, &tile_assets, &tile_mappers);
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
			RenderDistanceVisibility::default().with_offset(Vec3::new(
				tile_to_world_distance(Chunk::SIZE as i32 / 2),
				0.,
				tile_to_world_distance(Chunk::SIZE as i32 / 2),
			)),
			Collider::trimesh_with_flags(col_verts, col_indicies, TriMeshFlags::MERGE_DUPLICATE_VERTICES),
		));
	}
}

fn despawn_map(
	mut commands: Commands,
	mut heightmap: ResMut<Map>,
	cfg: Res<GenerationConfig>,
	map: Res<PhosMap>,
	chunks: Query<Entity, With<PhosChunk>>,
) {
	if !map.regenerate {
		return;
	}
	for chunk in chunks.iter() {
		commands.entity(chunk).despawn();
	}

	*heightmap = generate_heightmap(&cfg, 4);
}
