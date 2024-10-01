use bevy::{
	ecs::world::CommandQueue, prelude::*, tasks::AsyncComputeTaskPool, transform::commands, utils::futures,
	window::PrimaryWindow,
};
use bevy_asset_loader::loading_state::{
	config::{ConfigureLoadingState, LoadingStateConfig},
	LoadingStateAppExt,
};
use shared::{resources::TileUnderCursor, sets::GameplaySet, states::AssetLoadState};
use world_generation::{hex_utils::HexCoord, prelude::Map};

use crate::{
	assets::{unit_asset::UnitAssetPlugin, unit_database::UnitDatabase},
	components::{Path, PathTask, Target, Unit},
	units_debug_plugin::UnitsDebugPlugin,
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(UnitAssetPlugin);

		#[cfg(debug_assertions)]
		app.add_plugins(UnitsDebugPlugin);

		// app.configure_loading_state(LoadingStateConfig::new(AssetLoadState::Loading).load_collection::<UnitDatabase>());

		app.add_systems(Update, units_control.in_set(GameplaySet));
		app.add_systems(Update, move_unit.in_set(GameplaySet));
		app.add_systems(FixedPreUpdate, (calculate_path, resolve_path_task).in_set(GameplaySet));
	}
}

fn units_control(tile_under_cursor: Res<TileUnderCursor>) {}

fn move_unit(
	mut units: Query<(&mut Transform, &mut Path, Entity), With<Unit>>,
	time: Res<Time>,
	map: Res<Map>,
	mut commands: Commands,
) {
	for (mut t, mut path, entity) in units.iter_mut() {
		if path.1 >= path.0.len() {
			commands.entity(entity).remove::<Path>();
			continue;
		}
		let p = path.0[path.1];
		let d = p - t.translation;
		if d.length() < 0.1 {
			path.1 += 1;
			continue;
		}
		let vel = d.normalize() * 10.0 * time.delta_seconds();
		t.translation += vel;
		let coord = HexCoord::from_world_pos(t.translation);
		if map.is_in_bounds(&coord) {
			t.translation.y = map.sample_height(&coord);
		}
	}
}

fn calculate_path(
	units: Query<(&Transform, &Target, Entity), (With<Unit>, Without<PathTask>)>,
	map: Res<Map>,
	mut commands: Commands,
) {
	let pool = AsyncComputeTaskPool::get();
	for (transform, target, entity) in units.iter() {
		let from = transform.translation;
		let to = target.0;

		let task = pool.spawn(async move {
			let mut queue = CommandQueue::default();

			queue.push(move |world: &mut World| {
				//todo: calculate path
				world.entity_mut(entity).insert(Path(vec![from, to], 0));
			});
			return queue;
		});

		commands.entity(entity).insert(PathTask(task)).remove::<Target>();
	}
}

fn resolve_path_task(mut tasks: Query<(&mut PathTask, Entity), With<Unit>>, mut commands: Commands) {
	for (mut task, entity) in tasks.iter_mut() {
		if let Some(mut c) = futures::check_ready(&mut task.0) {
			commands.append(&mut c);
			commands.entity(entity).remove::<PathTask>();
		}
	}
}
