use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_lunex::prelude::MainUi;
use shared::sets::GameplaySet;
use shared::tags::MainCamera;
use world_generation::hex_utils::HexCoord;
use world_generation::prelude::Map;
use world_generation::states::GeneratorState;

use super::components::*;

pub struct PhosCameraPlugin;

impl Plugin for PhosCameraPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<PhosCamera>();
		app.register_type::<PhosOrbitCamera>();

		app.add_systems(PreStartup, setup);

		// app.add_systems(Update, rts_camera_system.in_set(GameplaySet));
		// app.add_systems(PostUpdate, limit_camera_bounds.in_set(GameplaySet));
		app.add_systems(Update, orbit_camera_upate.in_set(GameplaySet));

		app.add_systems(Update, init_bounds.run_if(in_state(GeneratorState::SpawnMap)));
		//Free Cam
		//app.add_systems(Update, (grab_mouse, (update_camera, update_camera_mouse).chain()));

		app.add_plugins(TemporalAntiAliasPlugin);
	}
}

fn init_bounds(
	mut commands: Commands,
	mut cam: Query<(&mut Transform, Entity), With<PhosCamera>>,
	heightmap: Res<Map>,
) {
	let (mut cam_t, cam_entity) = cam.single_mut();
	cam_t.translation = heightmap.get_center();
	commands
		.entity(cam_entity)
		.insert(CameraBounds::from_size(heightmap.get_world_size()))
		.insert(PhosOrbitCamera {
			target: heightmap.get_center_with_height(),
			..Default::default()
		});
}

fn setup(mut commands: Commands, mut msaa: ResMut<Msaa>) {
	commands
		.spawn((
			Camera3dBundle {
				transform: Transform::from_xyz(0., 30., 0.).looking_to(Vec3::NEG_Z, Vec3::Y),
				..default()
			},
			PhosCamera::default(),
			MainCamera,
			DepthPrepass,
			PhosOrbitCamera::default(),
			MainUi,
		))
		.insert(TemporalAntiAliasBundle::default())
		.insert(RenderLayers::layer(0));

	*msaa = Msaa::Off;
}

fn orbit_camera_upate(
	mut cam_query: Query<(&mut Transform, &PhosCamera, &mut PhosOrbitCamera, &CameraBounds)>,
	mut wheel: EventReader<MouseWheel>,
	mut mouse_motion: EventReader<MouseMotion>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut window_query: Query<&mut Window, With<PrimaryWindow>>,
	key: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	map: Res<Map>,
	#[cfg(debug_assertions)] mut gizmos: Gizmos,
) {
	let (mut transform, config, mut orbit, bounds) = cam_query.single_mut();
	let mut window = window_query.single_mut();

	let target = orbit.target;
	let mut cam_pos = target;

	//Apply Camera Dist
	cam_pos -= orbit.forward * orbit.distance;

	if mouse.pressed(MouseButton::Middle) {
		let mut orbit_move = Vec2::ZERO;
		for e in mouse_motion.read() {
			orbit_move += e.delta;
		}
		orbit_move *= config.pan_speed * time.delta_seconds() * -1.0;
		let rot_y = Quat::from_axis_angle(Vec3::Y, orbit_move.x);
		let right = orbit.forward.cross(Vec3::Y).normalize();
		let rot_x = Quat::from_axis_angle(right, orbit_move.y);
		orbit.forward = rot_x * rot_y * orbit.forward;
		// orbit.forward.y = orbit.forward.y.clamp(-0.9, 0.0);
		orbit.forward = orbit.forward.normalize();
		window.cursor.grab_mode = CursorGrabMode::Locked;
		window.cursor.visible = false;
	} else {
		window.cursor.grab_mode = CursorGrabMode::None;
		window.cursor.visible = true;
	}
	if key.pressed(KeyCode::KeyE) {
		let rot = Quat::from_axis_angle(Vec3::Y, f32::to_radians(config.speed) * time.delta_seconds());
		orbit.forward = rot * orbit.forward;
	} else if key.pressed(KeyCode::KeyQ) {
		let rot = Quat::from_axis_angle(Vec3::Y, f32::to_radians(-config.speed) * time.delta_seconds());
		orbit.forward = rot * orbit.forward;
	}

	let mut cam_move = Vec3::ZERO;

	if key.pressed(KeyCode::KeyA) {
		cam_move.x = 1.;
	} else if key.pressed(KeyCode::KeyD) {
		cam_move.x = -1.;
	}

	if key.pressed(KeyCode::KeyW) {
		cam_move.z = 1.;
	} else if key.pressed(KeyCode::KeyS) {
		cam_move.z = -1.;
	}

	let move_speed = if key.pressed(KeyCode::ShiftLeft) {
		config.speed * 2.0
	} else {
		config.speed
	};

	if cam_move != Vec3::ZERO {
		cam_move = cam_move.normalize();
		let move_fwd = Vec3::new(orbit.forward.x, 0., orbit.forward.z).normalize();
		let move_rot = Quat::from_rotation_arc(Vec3::NEG_Z, move_fwd);
		#[cfg(debug_assertions)]
		{
			gizmos.arrow(orbit.target, orbit.target + move_fwd, LinearRgba::WHITE.with_alpha(0.5));
			gizmos.arrow(orbit.target, orbit.target - (move_rot * cam_move), LinearRgba::BLUE);
		}
		orbit.target -= (move_rot * cam_move) * move_speed * time.delta_seconds();
		orbit.target.y = sample_ground(orbit.target, &map);

		orbit.target.x = orbit.target.x.clamp(bounds.min.x, bounds.max.x);
		orbit.target.z = orbit.target.z.clamp(bounds.min.y, bounds.max.y);
	}

	let mut scroll = 0.0;
	for e in wheel.read() {
		match e.unit {
			MouseScrollUnit::Line => scroll += e.y * 5.,
			MouseScrollUnit::Pixel => scroll += e.y,
		}
	}

	orbit.distance -= scroll * time.delta_seconds() * config.zoom_speed;
	orbit.distance = orbit.distance.clamp(config.min_height, config.max_height);

	// let ground_below_cam = sample_ground(cam_pos, &map) + config.min_height;
	// if cam_pos.y <= ground_below_cam {
	// 	cam_pos.y = ground_below_cam;
	// }

	// if cam_pos.y < target.y {
	// 	cam_pos.y = target.y;
	// }

	transform.translation = cam_pos;
	transform.look_at(target, Vec3::Y);
}

fn sample_ground(pos: Vec3, heightmap: &Map) -> f32 {
	let tile_under = HexCoord::from_world_pos(pos);
	let neighbors = heightmap.get_neighbors(&tile_under);
	let mut ground_height = if heightmap.is_in_bounds(&tile_under) {
		heightmap.sample_height(&tile_under)
	} else {
		heightmap.sealevel
	};

	for n in neighbors {
		if let Some(h) = n {
			if h > ground_height {
				ground_height = h;
			}
		}
	}
	if ground_height < heightmap.sealevel {
		ground_height = heightmap.sealevel;
	}
	return ground_height;
}
