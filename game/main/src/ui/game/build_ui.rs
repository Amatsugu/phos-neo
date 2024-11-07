use bevy::prelude::*;
use bevy_lunex::prelude::*;
use shared::tags::MainCamera;

pub struct BuildUiPlugin;

impl Plugin for BuildUiPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(UiDefaultPlugins)
			.add_plugins(UiDebugPlugin::<MainUi>::new());

		app.add_systems(PostStartup, setup_ui);
	}
}

fn setup_ui(mut commands: Commands, cam: Query<Entity, With<MainCamera>>, assets: Res<AssetServer>) {
	let c = cam.single();
	commands.entity(c).insert(MainUi);

	commands
		.spawn((
			UiTreeBundle::<MainUi> {
				tree: UiTree::new2d("BuildUi"),
				..default()
			},
			Name::new("Build UI"),
			SourceFromCamera,
		))
		.with_children(|ui| {
			ui.spawn((
				UiLink::<MainUi>::path("Root"),
				UiLayout::boundary()
					.pos1(Ab(20.0))
					.pos2(Rl(100.0) - Ab(20.0))
					.pack::<Base>(),
			));

			ui.spawn((
				UiLink::<MainUi>::path("Root/Rect"),
				UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack::<Base>(),
				UiImage2dBundle::from(assets.load("textures/world/test2.png")),
			));
		});
}
