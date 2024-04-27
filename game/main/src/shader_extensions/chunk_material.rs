use bevy::asset::{Asset, Handle};
use bevy::pbr::MaterialExtension;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::texture::Image;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {
	#[texture(100, dimension = "2d_array")]
	#[sampler(101)]
	pub array_texture: Handle<Image>,
}

impl MaterialExtension for ChunkMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/world/chunk.wgsl".into()
	}
}
