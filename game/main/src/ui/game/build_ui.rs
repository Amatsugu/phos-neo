use bevy::{prelude::*, render::view::RenderLayers};
use bevy_lunex::prelude::*;
use shared::tags::MainCamera;

pub struct BuildUiPlugin;

impl Plugin for BuildUiPlugin {
	fn build(&self, app: &mut App) {
		// app.add_plugins(UiDefaultPlugins)
		// 	.add_plugins(UiDebugPlugin::<MainUi>::new());

		app.add_systems(PostStartup, setup_ui);
	}
}

fn setup_ui(mut commands: Commands, assets: Res<AssetServer>) {
	commands
		.spawn((
			Camera2dBundle {
				transform: Transform::from_xyz(0.0, 0.0, 1000.0),
				..default()
			},
			MainUi,
		))
		.insert(RenderLayers::layer(1));

	commands
		.spawn((
			UiTreeBundle::<MainUi> {
				tree: UiTree::new2d("BuildUi"),
				..default()
			},
			Name::new("Build UI"),
			SourceFromCamera,
			RenderLayers::layer(1),
		))
		.with_children(|ui| {
			ui.spawn((
				UiLink::<MainUi>::path("Root"),
				UiLayout::boundary()
					.pos1(Ab(20.0))
					.pos2(Rl(100.0) - Ab(20.0))
					.pack::<Base>(),
				RenderLayers::layer(1),
			));

			ui.spawn((
				UiLink::<MainUi>::path("Root/Rect"),
				UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack::<Base>(),
				UiImage2dBundle::from(assets.load("textures/world/test2.png")),
				RenderLayers::layer(1),
			));
		});
}
