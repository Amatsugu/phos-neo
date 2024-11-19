use std::f32::consts::E;

use bevy::{ecs::world::CommandQueue, gltf::GltfMesh, prelude::*, window::PrimaryWindow};
use bevy_asset_loader::loading_state::{
	config::{ConfigureLoadingState, LoadingStateConfig},
	LoadingStateAppExt,
};
use bevy_rapier3d::{parry::transformation::utils::transform, pipeline::QueryFilter, plugin::RapierContext};
use shared::{
	despawn::Despawn,
	events::TileModifiedEvent,
	resources::TileUnderCursor,
	states::{AssetLoadState, GameplayState},
	tags::MainCamera,
};
use world_generation::{
	heightmap, hex_utils::HexCoord, map::map::Map, prelude::GenerationConfig, states::GeneratorState,
};

use crate::{
	assets::{
		building_asset::{BuildingAsset, BuildingAssetPlugin},
		building_database::BuildingDatabase,
	},
	build_queue::{BuildQueue, QueueEntry},
	buildings_map::{BuildingEntry, BuildingMap},
	prelude::Building,
};

pub struct BuildingPugin;

impl Plugin for BuildingPugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(BuildQueue::default());
		app.add_plugins(BuildingAssetPlugin);

		app.configure_loading_state(
			LoadingStateConfig::new(AssetLoadState::Loading).load_collection::<BuildingDatabase>(),
		);

		app.add_systems(Update, init.run_if(in_state(AssetLoadState::Loading)));
		app.add_systems(
			Update,
			hq_placement.run_if(in_state(GameplayState::PlaceHQ).and_then(in_state(GeneratorState::Idle))),
		);
		app.add_systems(
			PreUpdate,
			prepare_building_map.run_if(in_state(GeneratorState::SpawnMap)),
		);
		app.add_systems(Update, regernerate.run_if(in_state(GeneratorState::Regenerate)));
		app.add_systems(
			PostUpdate,
			update_building_heights.run_if(in_state(GeneratorState::Idle)),
		);

		app.add_systems(PreUpdate, process_build_queue.run_if(in_state(GameplayState::Playing)));
	}
}

fn prepare_building_map(mut commands: Commands, cfg: Res<GenerationConfig>) {
	commands.insert_resource(BuildingMap::new(cfg.size));
}

fn regernerate(mut commands: Commands, buildings: Query<Entity, With<Building>>, cfg: Res<GenerationConfig>) {
	for e in buildings.iter() {
		commands.entity(e).despawn();
	}
	commands.insert_resource(BuildingMap::new(cfg.size));
}

#[derive(Resource)]
struct IndicatorCube(Handle<Mesh>, Handle<StandardMaterial>);

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let cube = Cuboid::from_size(Vec3::splat(1.));
	let mesh_handle = meshes.add(cube);
	let mat_handle = materials.add(Color::WHITE);
	commands.insert_resource(IndicatorCube(mesh_handle, mat_handle));
}

fn hq_placement(
	mut commands: Commands,
	mouse: Res<ButtonInput<MouseButton>>,
	tile_under_cursor: Res<TileUnderCursor>,
	map: Res<Map>,
	indicator: Res<IndicatorCube>,
	mut build_queue: ResMut<BuildQueue>,
	mut next_state: ResMut<NextState<GameplayState>>,
) {
	if let Some(contact) = tile_under_cursor.0 {
		let positions = map.hex_select(&contact.tile, 3, true, |pos, h, _| pos.to_world(h));
		show_indicators(positions, &mut commands, &indicator);

		if mouse.just_pressed(MouseButton::Left) {
			build_queue.queue.push(QueueEntry {
				building: 0.into(),
				pos: contact.tile,
			});

			next_state.set(GameplayState::Playing);
		}
	}
}

fn show_indicators(positions: Vec<Vec3>, commands: &mut Commands, indicator: &IndicatorCube) {
	for p in positions {
		commands.spawn((
			PbrBundle {
				mesh: indicator.0.clone(),
				material: indicator.1.clone(),
				transform: Transform::from_translation(p),
				..default()
			},
			Despawn,
		));
	}
}

fn process_build_queue(
	mut queue: ResMut<BuildQueue>,
	mut commands: Commands,
	db: Res<BuildingDatabase>,
	building_assets: Res<Assets<BuildingAsset>>,
	gltf_assets: Res<Assets<Gltf>>,
	gltf_meshes: Res<Assets<GltfMesh>>,
	mut building_map: ResMut<BuildingMap>,
	heightmap: Res<Map>,
) {
	for item in &queue.queue {
		let handle = &db.buildings[item.building.0];
		if let Some(building) = building_assets.get(handle.id()) {
			let h = heightmap.sample_height(&item.pos);
			println!("Spawning {} at {}", building.name, item.pos);
			if let Some(gltf) = gltf_assets.get(building.prefab.id()) {
				let e = building.spawn(item.pos.to_world(h), Quat::IDENTITY, gltf, &mut commands, &gltf_meshes);
				if let Some(b) = e {
					building_map.add_building(BuildingEntry::new(item.pos, b));
				}
			} else {
				warn!("Failed to spawn building");
			}
		}
	}
	queue.queue.clear();
}

fn update_building_heights(
	mut tile_updates: EventReader<TileModifiedEvent>,
	building_map: Res<BuildingMap>,
	mut commands: Commands,
) {
	for event in tile_updates.read() {
		match event {
			TileModifiedEvent::HeightChanged(coord, new_height) => {
				if let Some(building) = building_map.get_building(coord) {
					let mut queue = CommandQueue::default();
					let e = building.entity.clone();
					let h = *new_height;
					queue.push(move |world: &mut World| {
						let mut emut = world.entity_mut(e);
						if let Some(mut transform) = emut.get_mut::<Transform>() {
							transform.translation.y = h;
						}
					});

					commands.append(&mut queue);
				}
			}
			_ => (),
		}
	}
}
