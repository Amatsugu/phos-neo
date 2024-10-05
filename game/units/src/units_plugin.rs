use std::collections::HashMap;

use bevy::{ecs::world::CommandQueue, prelude::*, tasks::AsyncComputeTaskPool, utils::futures};
use pathfinding::prelude::astar;
use shared::{resources::TileUnderCursor, sets::GameplaySet};
use world_generation::{hex_utils::HexCoord, prelude::Map};

use crate::{
	assets::unit_asset::UnitAssetPlugin,
	components::{Path, PathTask, Target, Unit},
	nav_data::NavData,
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
		app.add_systems(
			FixedPreUpdate,
			(dispatch_path_requests, resolve_path_task).in_set(GameplaySet),
		);
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

fn dispatch_path_requests(
	units: Query<(&Transform, &Target, Entity), (With<Unit>, Without<PathTask>)>,
	map: Res<Map>,
	mut commands: Commands,
) {
	if units.is_empty() {
		return;
	}
	let mut groups: HashMap<HexCoord, Vec<PathRequest>> = HashMap::new();

	for (transform, target, entity) in units.iter() {
		let req = PathRequest {
			entity,
			from: HexCoord::from_world_pos(transform.translation),
		};
		if let Some(group) = groups.get_mut(&target.0) {
			group.push(req);
		} else {
			groups.insert(target.0, vec![req]);
		}
	}

	//todo: only generate when map is changed
	let nav_data = NavData::build(&map);

	let pool = AsyncComputeTaskPool::get();
	for (target, units) in groups {
		let destinations = get_end_points(&target, units.len(), &map);
		let mut i = 0;
		for req in units {
			let d = nav_data.clone();
			let dst = destinations[i];
			i += 1;
			let task = pool.spawn(async move {
				#[cfg(feature = "tracing")]
				let _path_span = info_span!("Path Finding").entered();
				if let Some(path) = calculate_path(&req.from, &dst, d) {
					let mut queue = CommandQueue::default();
					queue.push(move |world: &mut World| {
						world.entity_mut(req.entity).insert(path);
					});
					return Some(queue);
				}
				return None;
			});
			commands
				.entity(req.entity)
				.insert(PathTask(task))
				.remove::<Target>()
				.remove::<Path>();
		}
	}
}

fn get_end_points(coord: &HexCoord, count: usize, map: &Map) -> Vec<HexCoord> {
	let mut result = Vec::with_capacity(count);
	if count == 1 {
		return vec![*coord];
	}
	result.push(*coord);
	let mut r = 1;
	while result.len() < count {
		let tiles = HexCoord::select_ring(coord, r);
		let needed = count - result.len();
		if needed >= tiles.len() {
			for t in tiles {
				if map.is_in_bounds(&t) {
					result.push(t);
				}
			}
		} else {
			for i in 0..needed {
				let t = tiles[i];
				if map.is_in_bounds(&t) {
					result.push(t);
				}
			}
		}
		r += 1;
	}

	return result;
}

fn resolve_path_task(mut tasks: Query<(&mut PathTask, Entity), With<Unit>>, mut commands: Commands) {
	for (mut task, entity) in tasks.iter_mut() {
		if let Some(c) = futures::check_ready(&mut task.0) {
			if let Some(mut queue) = c {
				commands.append(&mut queue);
			}
			commands.entity(entity).remove::<PathTask>();
		}
	}
}

fn calculate_path(from: &HexCoord, to: &HexCoord, nav: NavData) -> Option<Path> {
	let path = astar(
		from,
		|n| nav.get_neighbors(n),
		|n| nav.get(n).calculate_heuristic(to),
		|n| n == to,
	);
	if let Some((nodes, _cost)) = path {
		let result: Vec<_> = nodes.iter().map(|f| f.to_world(nav.get_height(f))).collect();
		return Some(Path(result, 1));
	}
	return None;
}

struct PathRequest {
	pub entity: Entity,
	pub from: HexCoord,
}
