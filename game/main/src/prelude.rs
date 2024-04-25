use bevy::asset::Handle;

use bevy::prelude::{Component, Image, Resource};

#[derive(Resource)]
pub struct ChunkAtlas {
	pub handle: Handle<Image>,
	pub is_loaded: bool,
}

#[derive(Resource, Default)]
pub struct PhosMap {
	pub ready: bool,
	pub regenerate: bool,
}

#[derive(Component)]
pub struct PhosChunk;
