use bevy::prelude::*;

use crate::prefab_defination::RotationAnimation;

pub struct SimpleAnimationPlugin;

impl Plugin for SimpleAnimationPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, rotate);
	}
}

fn rotate(mut query: Query<(&mut Transform, &RotationAnimation)>, time: Res<Time>) {
	for (mut transform, rot) in query.iter_mut() {
		let cur_rot = transform.rotation;
		transform.rotation = cur_rot * Quat::from_axis_angle(rot.axis, rot.speed.to_radians() * time.delta_secs());
	}
}
