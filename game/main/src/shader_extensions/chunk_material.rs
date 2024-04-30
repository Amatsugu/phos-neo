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

	// fn vertex_shader() -> ShaderRef {
	// 	"shaders/world/chunk_packed.wgsl".into()
	// }

	// fn specialize(
	// 	_pipeline: &bevy::pbr::MaterialExtensionPipeline,
	// 	descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
	// 	layout: &bevy::render::mesh::MeshVertexBufferLayout,
	// 	_key: bevy::pbr::MaterialExtensionKey<Self>,
	// ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
	// 	let vertex_layout = layout.get_layout(&[
	// 		// Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
	// 		// Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
	// 		// Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
	// 		ATTRIBUTE_PACKED_VERTEX_DATA.at_shader_location(7),
	// 		ATTRIBUTE_VERTEX_HEIGHT.at_shader_location(8),
	// 	])?;
	// 	descriptor.vertex.buffers = vec![vertex_layout];
	// 	Ok(())
	// }
}
