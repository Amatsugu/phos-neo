use bevy::asset::Handle;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::prelude::{Component, Image, Resource};
use bevy::reflect::Reflect;

use crate::shader_extensions::chunk_material::ChunkMaterial;

#[derive(Resource)]
pub struct ChunkAtlas {
	pub handle: Handle<Image>,
	pub chunk_material_handle: Handle<ExtendedMaterial<StandardMaterial, ChunkMaterial>>,
	pub water_material: Handle<StandardMaterial>,
	pub is_loaded: bool,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PhosMap {
	pub ready: bool,
	pub regenerate: bool,
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
