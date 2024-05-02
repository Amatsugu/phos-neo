use crate::prelude::PhosCamera;
use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::input::mouse::MouseMotion;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub mod prelude;

pub struct PhosCameraPlugin;

impl Plugin for PhosCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
			.add_systems(Update, (grab_mouse, (update_camera, update_camera_mouse).chain()));

		app.add_plugins(TemporalAntiAliasPlugin);
	}
}

fn setup(mut commands: Commands, mut msaa: ResMut<Msaa>) {
	commands
		.spawn((
			Camera3dBundle {
				transform: Transform::from_xyz(0., 30., 0.).looking_at(Vec3::new(1000., 0., 1000.), Vec3::Y),
				..default()
			},
			PhosCamera {
				speed: 100.,
				..default()
			},
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
	mut cam_query: Query<(&mut Transform, &PhosCamera)>,
	key: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	let (mut cam, cam_cfg) = cam_query.single_mut();
	let mut cam_move = Vec3::ZERO;

	if key.pressed(KeyCode::KeyA) {
		cam_move.x = -1.;
	} else if key.pressed(KeyCode::KeyD) {
		cam_move.x = 1.;
	}

	if key.pressed(KeyCode::KeyW) {
		cam_move.z = 1.;
	} else if key.pressed(KeyCode::KeyS) {
		cam_move.z = -1.;
	}

	cam.translation += cam_move.normalize() * cam_cfg.speed * time.delta_seconds();
}
