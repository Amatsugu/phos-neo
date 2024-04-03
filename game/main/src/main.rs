use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod phos;
mod prelude;

use phos::PhosGamePlugin;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Phos".into(),
					name: Some("phos".into()),
					resolution: (1920.0, 1080.0).into(),
					resizable: true,
					present_mode: PresentMode::AutoNoVsync,
					..default()
				}),
				..default()
			}),
			WorldInspectorPlugin::new(),
			PhosGamePlugin,
		))
		.run();
}
