use bevy::asset::{Asset, Handle};
use bevy::pbr::{Material, MaterialExtension, OpaqueRendererMethod};
use bevy::prelude::Mesh;
use bevy::reflect::TypePath;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::texture::Image;
use world_generation::consts::{ATTRIBUTE_PACKED_VERTEX_DATA, ATTRIBUTE_VERTEX_HEIGHT};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {
	#[texture(100, dimension = "2d_array")]
	#[sampler(101)]
	pub array_texture: Handle<Image>,
}

impl Material for ChunkMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/world/chunk_packed.wgsl".into()
	}

	fn vertex_shader() -> ShaderRef {
		"shaders/world/chunk_packed.wgsl".into()
	}

	fn prepass_vertex_shader() -> ShaderRef {
		"shaders/world/chunk_packed.wgsl".into()
	}

	fn deferred_vertex_shader() -> ShaderRef {
		"shaders/world/chunk_packed.wgsl".into()
	}

	fn opaque_render_method(&self) -> bevy::pbr::OpaqueRendererMethod {
		return OpaqueRendererMethod::Auto;
	}

	fn specialize(
		_pipeline: &bevy::pbr::MaterialPipeline<Self>,
		descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
		layout: &MeshVertexBufferLayoutRef,
		_key: bevy::pbr::MaterialPipelineKey<Self>,
	) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
		let vertex_layout = layout.0.get_layout(&[
			// Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
			// Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
			// Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
			ATTRIBUTE_PACKED_VERTEX_DATA.at_shader_location(7),
			ATTRIBUTE_VERTEX_HEIGHT.at_shader_location(8),
		])?;
		descriptor.vertex.buffers = vec![vertex_layout];
		Ok(())
	}

	// fn specialize(
	// 	descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
	// 	layout: &MeshVertexBufferLayoutRef,
	// 	_key: bevy::pbr::MaterialPipelineKey<Self>,
	// ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
	// 	let vertex_layout = layout.0.get_layout(&[
	// 		Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
	// 		Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
	// 		Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
	// 		ATTRIBUTE_PACKED_VERTEX_DATA.at_shader_location(7),
	// 		ATTRIBUTE_VERTEX_HEIGHT.at_shader_location(8),
	// 	])?;
	// 	descriptor.vertex.buffers = vec![vertex_layout];
	// 	Ok(())
	// }
}
