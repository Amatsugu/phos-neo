use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::texture::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use phos::PhosGamePlugin;

mod camera_system;
mod map_init;
mod phos;
mod prelude;
mod shader_extensions;
mod utlis;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Phos".into(),
						name: Some("phos".into()),
						// resolution: (1920., 1080.).into(),
						present_mode: PresentMode::AutoNoVsync,
						mode: bevy::window::WindowMode::BorderlessFullscreen,
						..default()
					}),
					..default()
				})
				.set(ImagePlugin {
					default_sampler: ImageSamplerDescriptor {
						address_mode_u: ImageAddressMode::Repeat,
						address_mode_v: ImageAddressMode::Repeat,
						mag_filter: ImageFilterMode::Nearest,
						..default()
					},
				}),
			WorldInspectorPlugin::new(),
			WireframePlugin,
			PhosGamePlugin,
		))
		.run();
}
