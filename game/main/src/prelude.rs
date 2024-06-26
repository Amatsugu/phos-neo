use bevy::asset::Handle;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::prelude::{Component, Image, Resource};

use crate::shader_extensions::chunk_material::ChunkMaterial;
use crate::shader_extensions::water_material::WaterMaterial;

#[derive(Resource, Default)]
pub struct ChunkAtlas {
	pub handle: Handle<Image>,
	pub chunk_material_handle: Handle<ExtendedMaterial<StandardMaterial, ChunkMaterial>>,
	pub water_material: Handle<ExtendedMaterial<StandardMaterial, WaterMaterial>>,
	pub is_loaded: bool,
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
}

impl PhosChunkRegistry {
	pub fn new(size: usize) -> Self {
		return Self {
			chunks: Vec::with_capacity(size),
		};
	}
}

#[derive(Component)]
pub struct RebuildChunk;
