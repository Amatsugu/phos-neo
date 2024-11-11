use bevy::asset::Handle;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::prelude::{Component, Image, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;
use world_generation::biome_painter::BiomePainterAsset;

use crate::shader_extensions::chunk_material::ChunkMaterial;
use crate::shader_extensions::water_material::WaterMaterial;

#[derive(AssetCollection, Resource, Default)]
pub struct PhosAssets {
	#[asset(key = "chunk_atlas")]
	pub handle: Handle<Image>,
	pub chunk_material_handle: Handle<ExtendedMaterial<StandardMaterial, ChunkMaterial>>,
	pub water_material: Handle<ExtendedMaterial<StandardMaterial, WaterMaterial>>,
}

#[derive(Component)]
pub struct PhosChunk {
	pub index: usize,
}

impl PhosChunk {
	pub fn new(index: usize) -> Self {
		return Self { index };
	}
}

#[derive(Resource, Default)]
pub struct PhosChunkRegistry {
	pub chunks: Vec<Entity>,
	pub waters: Vec<Entity>,
}

impl PhosChunkRegistry {
	pub fn new(size: usize) -> Self {
		return Self {
			chunks: Vec::with_capacity(size),
			waters: Vec::with_capacity(size),
		};
	}
}

#[derive(Component)]
pub struct RebuildChunk;
