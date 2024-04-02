use bevy::asset::Handle;
use bevy::prelude::{Image, Resource};

#[derive(Resource)]
pub struct ChunkAtlas {
	pub handle: Handle<Image>,
}
