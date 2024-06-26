use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};
use shared::{
	despawn::Despawn,
	states::{GameplayState, MenuState},
	tags::MainCamera,
};
use world_generation::{hex_utils::HexCoord, map::map::Map};

use crate::build_queue::{self, BuildQueue, QueueEntry};

pub struct BuildingPugin;

impl Plugin for BuildingPugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(BuildQueue::default());

		app.add_systems(Startup, init);
		app.add_systems(Update, hq_placement.run_if(in_state(GameplayState::PlaceHQ)));

		app.add_systems(PreUpdate, process_build_queue.run_if(in_state(GameplayState::Playing)));
	}
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
	cam_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
	mut commands: Commands,
	window: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<ButtonInput<MouseButton>>,
	rapier_context: Res<RapierContext>,
	map: Res<Map>,
	indicator: Res<IndicatorCube>,
	mut build_queue: ResMut<BuildQueue>,
	next_state: ResMut<NextState<GameplayState>>,
) {
	let win = window.single();
	let (cam_transform, camera) = cam_query.single();
	let Some(cursor_pos) = win.cursor_position() else {
		return;
	};

	let Some(cam_ray) = camera.viewport_to_world(cam_transform, cursor_pos) else {
		return;
	};

	let collision = rapier_context.cast_ray(
		cam_ray.origin,
		cam_ray.direction.into(),
		500.,
		true,
		QueryFilter::only_fixed(),
	);

	if let Some((_e, dist)) = collision {
		let contact_point = cam_ray.get_point(dist);
		let contact_coord = HexCoord::from_world_pos(contact_point);
		let positions = map.hex_select(&contact_coord, 3, true, |pos, h, _| pos.to_world(h));
		show_indicators(positions, &mut commands, &indicator);

		if mouse.just_pressed(MouseButton::Left) {
			build_queue.queue.push(QueueEntry {
				building: 0.into(),
				pos: contact_coord,
			});
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

fn process_build_queue(mut queue: ResMut<BuildQueue>) {}
