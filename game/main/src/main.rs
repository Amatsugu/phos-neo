use bevy::prelude::*;
use bevy::render::texture::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod phos;
mod prelude;
mod shader_extensions;
use phos::PhosGamePlugin;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Phos".into(),
						name: Some("phos".into()),
						resolution: (1920., 1080.).into(),
						present_mode: PresentMode::AutoNoVsync,
						// mode: bevy::window::WindowMode::BorderlessFullscreen,
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
			PhosGamePlugin,
		))
		.run();
}
