use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self};
use bevy_rapier3d::rapier::crossbeam::deque::Steal;
use image::{ImageBuffer, Rgba};
use world_generation::map::biome_map::{self, BiomeMap};
use world_generation::map::map_utils::{render_biome_map, render_biome_noise_map, update_map};
use world_generation::{map::map_utils::render_map, prelude::Map, states::GeneratorState};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<UIState>();

		app.add_systems(PostUpdate, prepare_image.run_if(in_state(GeneratorState::SpawnMap)));
		app.add_systems(
			Update,
			(render_map_ui, update_map_render).run_if(in_state(GeneratorState::Idle)),
		);
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
	pub target_map_type: MapDisplayType,
	pub cur_map_type: MapDisplayType,
}

impl Default for UIState {
	fn default() -> Self {
		Self {
			is_open: true,
			target_map_type: default(),
			cur_map_type: default(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum MapDisplayType {
	#[default]
	HeightMap,
	Biomes,
	BiomeNoise,
	BiomeNoiseTemp,
	BiomeNoiseContinent,
	BiomeNoiseMoisture,
}

fn render_map_ui(image: Res<MapImage>, heightmap: Res<Map>, biome_map: Res<BiomeMap>, mut contexts: EguiContexts, mut state: ResMut<UIState>) {
	let id = contexts.add_image(image.0.clone_weak());

	let mut map_type = state.target_map_type;
	let ctx = contexts.ctx_mut();
	egui::Window::new("Map").open(&mut state.is_open).show(ctx, |ui| {
		ui.label("Map Test");
		egui::ComboBox::from_label("Display Type")
			.selected_text(format!("{:?}", map_type))
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut map_type, MapDisplayType::HeightMap, "Heightmap");
				ui.selectable_value(&mut map_type, MapDisplayType::Biomes, "Biomes");
				ui.selectable_value(&mut map_type, MapDisplayType::BiomeNoise, "Biome Noise");
				ui.selectable_value(
					&mut map_type,
					MapDisplayType::BiomeNoiseTemp,
					"Biome Noise: Tempurature",
				);
				ui.selectable_value(
					&mut map_type,
					MapDisplayType::BiomeNoiseContinent,
					"Biome Noise: Continent",
				);
				ui.selectable_value(
					&mut map_type,
					MapDisplayType::BiomeNoiseMoisture,
					"Biome Noise: Moisture",
				);
			});

		ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
			id,
			[512.0, 512.0],
		)));

		if ui.button("Save Image").clicked() {
			let img = get_map_image(&heightmap, &biome_map, map_type);
			_ = img.save(format!("{:?}.png", map_type));
		}
	});

	state.target_map_type = map_type;
}

fn update_map_render(
	mut state: ResMut<UIState>,
	mut images: ResMut<Assets<Image>>,
	heightmap: Res<Map>,
	biome_map: Res<BiomeMap>,
	image: Res<MapImage>,
) {
	if state.cur_map_type == state.target_map_type {
		return;
	}

	let result = get_map_image(&heightmap, &biome_map, state.target_map_type);
	images.insert(
		image.0.id(),
		Image::from_dynamic(result.into(), true, RenderAssetUsages::RENDER_WORLD),
	);

	state.cur_map_type = state.target_map_type;
}

fn get_map_image(heightmap: &Map, biome_map: &BiomeMap, map_type: MapDisplayType) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	return match map_type {
		MapDisplayType::HeightMap => render_map(&heightmap, 0.1),
		MapDisplayType::Biomes => render_biome_map(&heightmap, &biome_map),
		MapDisplayType::BiomeNoise => render_biome_noise_map(&biome_map, Vec3::ONE),
		MapDisplayType::BiomeNoiseTemp => render_biome_noise_map(&biome_map, Vec3::X),
		MapDisplayType::BiomeNoiseContinent => render_biome_noise_map(&biome_map, Vec3::Y),
		MapDisplayType::BiomeNoiseMoisture => render_biome_noise_map(&biome_map, Vec3::Z),
	};
}
