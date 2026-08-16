use bevy::asset::Asset;
use bevy::pbr::MaterialExtension;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
pub struct WaterMaterial
{
	#[uniform(100)]
	pub settings: WaterSettings,
}

#[derive(Debug, Clone, ShaderType, Reflect)]
pub struct WaterSettings
{
	pub surface_level: f32,
	pub scale: f32,
	pub f_power: f32,
	pub deep_color: LinearRgba,
}

impl Default for WaterSettings
{
	fn default() -> Self
	{
		Self {
			scale: 1.0,
			f_power: 2.0,
			surface_level: 0.0,
			deep_color: default(),
		}
	}
}

impl MaterialExtension for WaterMaterial
{
	fn fragment_shader() -> ShaderRef
	{
		"shaders/world/water.wgsl".into()
	}
}
