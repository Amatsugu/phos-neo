use bevy::asset::Asset;
use bevy::math::VectorSpace;
use bevy::pbr::MaterialExtension;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
pub struct WaterMaterial {
	#[uniform(100)]
	pub settings: WaterSettings,
}

#[derive(Debug, Clone, ShaderType, Reflect)]
pub struct WaterSettings {
	pub offset: f32,
	pub scale: f32,
	pub deep_color: Vec3,
}

impl Default for WaterSettings {
	fn default() -> Self {
		Self {
			offset: 0.0,
			scale: 1.0,
			deep_color: Vec3::ZERO,
		}
	}
}

impl MaterialExtension for WaterMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/world/water.wgsl".into()
	}

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
