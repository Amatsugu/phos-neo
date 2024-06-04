use bevy::prelude::*;

use crate::camera_system::components::PhosCamera;

pub struct RenderDistancePlugin;

impl Plugin for RenderDistancePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.register_type::<RenderDistanceSettings>();
		app.add_systems(PostUpdate, render_distance_system)
			.insert_resource(RenderDistanceSettings::default());
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RenderDistanceSettings {
	pub render_distance: f32,
}

impl RenderDistanceSettings {
	pub fn new(distance: f32) -> Self {
		return Self {
			render_distance: distance,
		};
	}
}

impl Default for RenderDistanceSettings {
	fn default() -> Self {
		Self::new(500.)
	}
}

#[derive(Component)]
pub struct RenderDistanceVisibility {
	pub offset: Vec3,
}

impl RenderDistanceVisibility {
	pub fn with_offset(mut self, offset: Vec3) -> Self {
		self.offset = offset;
		return self;
	}
}

impl Default for RenderDistanceVisibility {
	fn default() -> Self {
		Self { offset: Vec3::ZERO }
	}
}

fn render_distance_system(
	mut objects: Query<(&Transform, &mut Visibility, &RenderDistanceVisibility)>,
	camera_query: Query<&Transform, With<PhosCamera>>,
	settings: Res<RenderDistanceSettings>,
) {
	let camera = camera_query.single();
	for (t, mut vis, r) in objects.iter_mut() {
		let dist = (camera.translation - (t.translation + r.offset)).length();
		if settings.render_distance < dist {
			*vis = Visibility::Hidden;
		} else {
			*vis = Visibility::Visible;
		}
	}
}
