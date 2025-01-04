use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

pub struct BuildUiPlugin;

impl Plugin for BuildUiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostStartup, setup_ui);
	}
}

fn setup_ui(mut commands: Commands, assets: Res<AssetServer>, mut material: ResMut<Assets<StandardMaterial>>) {
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
					.pos1((Rl(0.), Rl(0.)))
					.pos2((Rl(100.), Rl(100.)))
					.pack::<Base>(),
			));

			ui.spawn((
				UiLink::<MainUi>::path("Root/MainRect"),
				UiLayout::window()
					.anchor(Anchor::BottomCenter)
					.size((Ab(800.), Rl(100.)))
					.pos((Rl(50.), Rl(100.)))
					.pack::<Base>(),
			));

			ui.spawn((
				UiLink::<MainUi>::path("Root/MainRect/Categories"),
				UiLayout::solid()
					.align_y(Align::END)
					.size((Rl(100.), Ab(30.)))
					.pack::<Base>(),
			));

			for i in 1..5 {
				let path = format!("Root/MainRect/Categories/Button{}", i);
				ui.spawn((
					UiLink::<MainUi>::path(path),
					UiLayout::div()
						.margin_x(Ab(5.))
						.height(Sizing::Max)
						.min_height(Ab(30.))
						.min_width(Sp(10.))
						.max_width(Sp(100.))
						.pack::<Base>(),
					UiImage2dBundle::from(assets.load("textures/world/test2.png")),
				));
			}
		});
}
