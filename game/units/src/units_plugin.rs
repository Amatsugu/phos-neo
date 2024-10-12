use std::collections::HashMap;

use bevy::{ecs::world::CommandQueue, prelude::*, tasks::AsyncComputeTaskPool, utils::futures};
use pathfinding::prelude::astar;
use shared::{events::TileModifiedEvent, resources::TileUnderCursor, sets::GameplaySet};
use world_generation::{hex_utils::HexCoord, prelude::Map, states::GeneratorState};

#[cfg(debug_assertions)]
use crate::units_debug_plugin::UnitsDebugPlugin;
use crate::{
	assets::unit_asset::UnitAssetPlugin,
	components::{Path, PathTask, PathTaskPending, Target, Unit},
	nav_data::NavData,
	resources::PathBatchId,
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<PathBatchId>();
		app.add_plugins(UnitAssetPlugin);

		#[cfg(debug_assertions)]
		app.add_plugins(UnitsDebugPlugin);

		// app.configure_loading_state(LoadingStateConfig::new(AssetLoadState::Loading).load_collection::<UnitDatabase>());
		app.add_systems(PostUpdate, build_navdata.run_if(in_state(GeneratorState::SpawnMap)));

		app.add_systems(Update, units_control.in_set(GameplaySet));
		app.add_systems(Update, (move_unit, update_navdata).in_set(GameplaySet));
		app.add_systems(
			FixedPreUpdate,
			(dispatch_path_requests, resolve_path_task).in_set(GameplaySet),
		);
	}
}

fn build_navdata(mut commands: Commands, map: Res<Map>) {
	let nav_data = NavData::build(&map);
	commands.insert_resource(nav_data);
}

fn update_navdata(mut tile_updates: EventReader<TileModifiedEvent>, mut nav_data: ResMut<NavData>) {
	for event in tile_updates.read() {
		match event {
			TileModifiedEvent::HeightChanged(coord, new_height) => {
				nav_data.update_tile(coord, *new_height, 1.0);
			}
			_ => (),
		}
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
	units: Query<(&Transform, &Target, Entity), With<Unit>>,
	map: Res<Map>,
	nav_data: Res<NavData>,
	mut batch_id: ResMut<PathBatchId>,
	mut commands: Commands,
) {
	if units.is_empty() {
		return;
	}
	let mut groups: HashMap<HexCoord, Vec<PathRequest>> = HashMap::new();

	#[cfg(feature = "tracing")]
	let _group_span = info_span!("Grouping").entered();
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
	#[cfg(feature = "tracing")]
	drop(_group_span);

	let pool = AsyncComputeTaskPool::get();
	for (target, units) in groups {
		let id = batch_id.0;
		batch_id.0 += 1;

		for req in &units {
			commands
				.entity(req.entity)
				.insert(PathTaskPending(id))
				.remove::<Target>();
		}

		let destinations = get_end_points(&target, units.len(), &map);
		let req = BatchPathRequest::new(units, destinations);

		#[cfg(feature = "tracing")]
		let _clone_span = info_span!("Nav Data Clone").entered();
		let local_nav_data = nav_data.clone();
		#[cfg(feature = "tracing")]
		drop(_clone_span);

		let batch_task = pool.spawn(async move {
			let mut i = 0;
			let mut queue = CommandQueue::default();
			for entitiy_req in req.entities {
				let dst = req.destination[i];
				i += 1;
				#[cfg(feature = "tracing")]
				let _path_span = info_span!("Path Finding").entered();
				if let Some(path) = calculate_path(&entitiy_req.from, &dst, &local_nav_data) {
					queue.push(move |world: &mut World| {
						let mut unit_e = world.entity_mut(entitiy_req.entity);

						if let Some(pending_task) = unit_e.get::<PathTaskPending>() {
							if pending_task.0 == id {
								unit_e.insert(path);
								unit_e.remove::<PathTaskPending>();
							}
						}
					});
				}
			}
			if queue.is_empty() {
				return None;
			}
			return Some(queue);
		});
		commands.spawn(PathTask(batch_task));
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

fn resolve_path_task(mut tasks: Query<(&mut PathTask, Entity)>, mut commands: Commands) {
	for (mut task, entity) in tasks.iter_mut() {
		if let Some(c) = futures::check_ready(&mut task.0) {
			if let Some(mut queue) = c {
				commands.append(&mut queue);
			}
			commands.entity(entity).despawn();
		}
	}
}

fn calculate_path(from: &HexCoord, to: &HexCoord, nav: &NavData) -> Option<Path> {
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

struct BatchPathRequest {
	pub entities: Vec<PathRequest>,
	pub destination: Vec<HexCoord>,
}

impl BatchPathRequest {
	pub fn new(entities: Vec<PathRequest>, dst: Vec<HexCoord>) -> Self {
		return Self {
			destination: dst,
			entities,
		};
	}
}
