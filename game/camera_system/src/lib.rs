use crate::prelude::PhosCamera;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub mod prelude {
	use bevy::prelude::Component;

	#[derive(Component, Default)]
	pub struct PhosCamera {
		pub min_height: f32,
		pub max_height: f32,
		pub speed: f32,
	}
}

pub struct PhosCameraPlugin;

impl Plugin for PhosCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
			.add_systems(Update, (update_camera, grab_mouse));
	}
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(-200., 300., -200.)
				.looking_at(Vec3::new(1000., 0., 1000.), Vec3::Y),
			..default()
		},
		PhosCamera {
			speed: 100.,
			..default()
		},
	));
}
fn update_camera(
	mut cam_query: Query<(&PhosCamera, &mut Transform)>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
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
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		move_vec += Vec3::NEG_Y;
	}
	if keyboard_input.pressed(KeyCode::Space) {
		move_vec += Vec3::Y;
	}
	if move_vec.length_squared() == 0. {
		return;
	}
	let rot = transform.rotation;
	transform.translation += (rot * move_vec.normalize()) * cam.speed * time.delta_seconds();
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
	let cam_rot = mouse_move.read().map(|event| event.delta).sum::<Vec2>() * time.delta_seconds();

	let (mut pitch, mut yaw, _) = transform.rotation.to_euler(EulerRot::XYZ);

	pitch -= cam_rot.y.to_radians() ;
	yaw -= cam_rot.x.to_radians() ;
	pitch = pitch.clamp(-1.54, 1.54);
	// if rot_x > PI && cam_rot.x < 2. * PI {
	// 	rot_x = PI;
	// }

	transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
}

fn grab_mouse(
	mut windows: Query<&mut Window>,
	mouse: Res<ButtonInput<MouseButton>>,
	key: Res<ButtonInput<KeyCode>>,
) {
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
