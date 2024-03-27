use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod phos;
use phos::PhosGamePlugin;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Phos".into(),
					name: Some("phos".into()),
					resolution: (1920.0, 1080.0).into(),
					resizable: false,
					enabled_buttons: bevy::window::EnabledButtons {
						maximize: false,
						..Default::default()
					},
					..default()
				}),
				..default()
			}),
			WireframePlugin,
			WorldInspectorPlugin::new(),
			PhosGamePlugin,
		))
		.run();
}
