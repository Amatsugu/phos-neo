use bevy::asset::{Asset, Handle};
use bevy::pbr::{MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::{Component, Image, Mesh, Resource, TypePath};
use bevy::render::mesh::{Indices, MeshVertexBufferLayout};
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use world_generation::prelude::ATTRIBUTE_TEXTURE_INDEX;

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

	// fn specialize(
	// 	_pipeline: &MaterialExtensionPipeline,
	// 	descriptor: &mut RenderPipelineDescriptor,
	// 	layout: &MeshVertexBufferLayout,
	// 	_key: MaterialExtensionKey<Self>,
	// ) -> Result<(), SpecializedMeshPipelineError> {
	// 	let vertex_layout = layout.get_layout(&[
	// 		Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
	// 		Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
	// 		Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
	// 		ATTRIBUTE_TEXTURE_INDEX.at_shader_location(3),
	// 	])?;
	// 	descriptor.vertex.buffers = vec![vertex_layout];
	// 	Ok(())
	// }
}
