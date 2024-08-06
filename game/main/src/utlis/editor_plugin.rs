use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self};
use world_generation::{map::map_utils::render_map, prelude::Map, states::GeneratorState};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<UIState>();

		app.add_systems(PostUpdate, prepare_image.run_if(in_state(GeneratorState::SpawnMap)));
		app.add_systems(Update, (render_map_ui).run_if(in_state(GeneratorState::Idle)));
	}
}

#[derive(Resource)]
struct MapImage(pub Handle<Image>);

pub fn prepare_image(mut images: ResMut<Assets<Image>>, heightmap: Res<Map>, mut commands: Commands) {
	let image = render_map(&heightmap, 0.1);
	let handle = images.add(Image::from_dynamic(image.into(), true, RenderAssetUsages::RENDER_WORLD));

	commands.insert_resource(MapImage(handle));
}

#[derive(Resource)]
struct UIState {
	pub is_open: bool,
}

impl Default for UIState {
	fn default() -> Self {
		Self { is_open: true }
	}
}

fn render_map_ui(image: Res<MapImage>, mut contexts: EguiContexts, mut state: ResMut<UIState>) {
	let id = contexts.add_image(image.0.clone_weak());

	let ctx = contexts.ctx_mut();
	egui::Window::new("Map").open(&mut state.is_open).show(ctx, |ui| {
		ui.label("Map Test");
		ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
			id,
			[512.0, 512.0],
		)));
	});
}
