use crate::prelude::PhosCamera;
use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use prelude::{CameraBounds, PhosCameraTargets};

pub mod prelude;

pub struct PhosCameraPlugin;

impl Plugin for PhosCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, setup);

		app.add_systems(Update, (rts_camera_system, apply_camera_height).chain());
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
				transform: Transform::from_xyz(0., 30., 0.)
					.with_rotation(Quat::from_axis_angle(Vec3::Y, (-90 as f32).to_radians())),
				..default()
			},
			PhosCamera {
				speed: 100.,
				zoom_speed: 20.,
				..default()
			},
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
	key: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut wheel: EventReader<MouseWheel>,
) {
	let (mut cam, cam_cfg, mut cam_targets) = cam_query.single_mut();
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

	let fwd = cam.forward();
	let fwd_quat = Quat::from_rotation_arc(Vec3::Z, fwd.into());
	cam_move = fwd_quat.mul_vec3(cam_move.normalize_or_zero()) * cam_cfg.speed * time.delta_seconds();

	for e in wheel.read() {
		match e.unit {
			MouseScrollUnit::Line => cam_targets.height -= e.y * 20.,
			MouseScrollUnit::Pixel => cam_targets.height -= e.y,
		}
	}

	cam_targets.height = cam_targets.height.clamp(cam_cfg.min_height, cam_cfg.max_height);

	cam.translation += cam_move;
}

fn apply_camera_height(mut cam_query: Query<(&mut Transform, &PhosCamera, &mut PhosCameraTargets)>, time: Res<Time>) {
	let (mut cam_t, cam_cfg, targets) = cam_query.single_mut();

	let movement = (cam_t.translation.y - targets.height) * time.delta_seconds();

	cam_t.translation.y -= movement;
}

fn limit_camera_bounds(mut cam_query: Query<(&mut Transform, &CameraBounds)>) {
	
}
