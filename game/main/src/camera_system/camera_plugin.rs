use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::plugin::RapierContext;
use world_generation::hex_utils::HexCoord;
use world_generation::prelude::Map;

use super::components::*;

pub struct PhosCameraPlugin;

impl Plugin for PhosCameraPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<PhosCamera>();

		app.add_systems(PreStartup, setup);

		app.add_systems(Update, rts_camera_system);
		app.add_systems(PostUpdate, limit_camera_bounds);
		//Free Cam
		//app.add_systems(Update, (grab_mouse, (update_camera, update_camera_mouse).chain()));

		app.add_plugins(TemporalAntiAliasPlugin);
	}
}

fn setup(mut commands: Commands, mut msaa: ResMut<Msaa>) {
	commands
		.spawn((
			Camera3dBundle {
				transform: Transform::from_xyz(0., 30., 0.).looking_to(Vec3::Z, Vec3::Y),
				..default()
			},
			PhosCamera::default(),
			PhosCameraTargets::default(),
		))
		.insert(ScreenSpaceAmbientOcclusionBundle::default())
		.insert(TemporalAntiAliasBundle::default());

	*msaa = Msaa::Off;
}
fn update_camera(
	mut cam_query: Query<(&PhosCamera, &mut Transform)>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	windows: Query<&Window>,
) {
	let window = windows.single();
	if window.cursor.grab_mode != CursorGrabMode::Locked {
		return;
	}
	let (cam, mut transform) = cam_query.single_mut();

	let mut move_vec = Vec3::ZERO;
	if keyboard_input.pressed(KeyCode::KeyA) {
		move_vec += Vec3::NEG_X;
	}
	if keyboard_input.pressed(KeyCode::KeyD) {
		move_vec += Vec3::X;
	}
	if keyboard_input.pressed(KeyCode::KeyW) {
		move_vec += Vec3::NEG_Z;
	}
	if keyboard_input.pressed(KeyCode::KeyS) {
		move_vec += Vec3::Z;
	}

	let rot = transform.rotation;
	move_vec = (rot * move_vec.normalize_or_zero()) * cam.speed * time.delta_seconds();

	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		move_vec += Vec3::from(transform.down());
	}
	if keyboard_input.pressed(KeyCode::Space) {
		move_vec += Vec3::from(transform.up());
	}

	transform.translation += move_vec.normalize_or_zero() * cam.speed * time.delta_seconds();
}

fn update_camera_mouse(
	mut cam_query: Query<&mut Transform, With<PhosCamera>>,
	mut mouse_move: EventReader<MouseMotion>,
	time: Res<Time>,
	windows: Query<&Window>,
) {
	let window = windows.single();
	if window.cursor.grab_mode != CursorGrabMode::Locked {
		return;
	}
	let mut transform = cam_query.single_mut();

	for ev in mouse_move.read() {
		let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
		match window.cursor.grab_mode {
			CursorGrabMode::None => (),
			_ => {
				// Using smallest of height or width ensures equal vertical and horizontal sensitivity
				pitch -= ev.delta.y.to_radians() * time.delta_seconds() * 5.;
				yaw -= ev.delta.x.to_radians() * time.delta_seconds() * 5.;
			}
		}

		pitch = pitch.clamp(-1.54, 1.54);

		// Order is important to prevent unintended roll
		transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
	}
}

fn grab_mouse(mut windows: Query<&mut Window>, mouse: Res<ButtonInput<MouseButton>>, key: Res<ButtonInput<KeyCode>>) {
	let mut window = windows.single_mut();

	if mouse.just_pressed(MouseButton::Middle) {
		window.cursor.visible = false;
		window.cursor.grab_mode = CursorGrabMode::Locked;
	}

	if key.just_pressed(KeyCode::Escape) {
		window.cursor.visible = true;
		window.cursor.grab_mode = CursorGrabMode::None;
	}
}

fn rts_camera_system(
	mut cam_query: Query<(&mut Transform, &PhosCamera, &mut PhosCameraTargets)>,
	mut wheel: EventReader<MouseWheel>,
	key: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	heightmap: Res<Map>,
) {
	let (mut cam, cam_cfg, mut cam_targets) = cam_query.single_mut();
	let mut cam_move = Vec3::ZERO;
	let mut cam_pos = cam.translation;

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

	if key.pressed(KeyCode::ShiftLeft) {
		cam_move = cam_move.normalize_or_zero() * cam_cfg.speed * time.delta_seconds() * 2.;
	} else {
		cam_move = cam_move.normalize_or_zero() * cam_cfg.speed * time.delta_seconds();
	}

	cam_pos -= cam_move;

	let mut scroll = 0.0;
	for e in wheel.read() {
		match e.unit {
			MouseScrollUnit::Line => scroll += e.y * 5.,
			MouseScrollUnit::Pixel => scroll += e.y,
		}
	}

	cam_targets.height -= scroll;
	if cam_targets.height > cam_cfg.max_height {
		cam_targets.height = cam_cfg.max_height;
	}

	let tile_under = HexCoord::from_world_pos(cam.translation);
	let neighbors = heightmap.get_neighbors(&tile_under);
	let mut ground_height = heightmap.sample_height(&tile_under);
	for n in neighbors {
		if let Some(h) = n {
			if h > ground_height {
				ground_height = h;
			}
		}
	}

	let min_height = ground_height + cam_cfg.min_height;

	if min_height != cam_targets.last_height {
		cam_targets.last_height = min_height;
		cam_targets.anim_time = 0.;
		cam_targets.rotate_time = 0.;
	}

	if scroll != 0. {
		cam_targets.anim_time = 0.;
		cam_targets.rotate_time = 0.;
		if cam_targets.height < min_height {
			cam_targets.height = min_height;
		}
	}

	let desired_height = if cam_targets.height < min_height {
		min_height
	} else {
		cam_targets.height
	};
	if cam_targets.anim_time < 1. {
		cam_targets.anim_time += time.delta_seconds() * cam_cfg.zoom_speed;
		cam_targets.anim_time = cam_targets.anim_time.min(1.);
	}
	cam_pos.y = f32::lerp(cam_pos.y, desired_height, cam_targets.anim_time);
	let t = cam_pos.y.remap(cam_cfg.min_height, cam_cfg.max_height, 0., 1.);

	if cam_targets.rotate_time < 1. {
		cam_targets.rotate_time += time.delta_seconds();
		cam_targets.rotate_time = cam_targets.rotate_time.min(1.);
	}
	let angle = cam_cfg.min_angle.lerp(cam_cfg.max_angle, t);
	let rot = Quat::from_axis_angle(Vec3::X, -angle);
	cam.rotation = rot;

	cam.translation = cam_pos;
}

fn limit_camera_bounds(mut cam_query: Query<(&mut Transform, &CameraBounds)>) {}
