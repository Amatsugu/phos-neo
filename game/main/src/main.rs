use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
#[cfg(debug_assertions)]
use bevy::window::WindowResolution;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use phos::PhosGamePlugin;

mod camera_system;
mod map_rendering;
mod phos;
mod prelude;
mod shader_extensions;
mod ui;
mod utlis;

fn main()
{
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Phos".into(),
						name: Some("phos".into()),
						#[cfg(debug_assertions)]
						resolution: WindowResolution::new(1920, 1080),
						present_mode: PresentMode::AutoNoVsync,
						#[cfg(not(debug_assertions))]
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
				})
				.set(AssetPlugin {
					#[cfg(not(debug_assertions))]
					watch_for_changes_override: Some(true),
					..Default::default()
				}),
			EguiPlugin::default(),
			WorldInspectorPlugin::new(),
			WireframePlugin::default(),
			PhosGamePlugin,
		))
		.run();
}
