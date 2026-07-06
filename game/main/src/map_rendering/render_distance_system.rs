use bevy::prelude::*;
use hex::{SHORT_DIAGONAL, prelude::Chunk};
use shared::tags::MainCamera;
use world_generation::states::GeneratorState;

pub struct RenderDistancePlugin;

impl Plugin for RenderDistancePlugin
{
	fn build(&self, app: &mut bevy::prelude::App)
	{
		app.register_type::<RenderDistanceSettings>();
		app.add_systems(
			PostUpdate,
			render_distance_system.run_if(in_state(GeneratorState::Idle)),
		)
		.insert_resource(RenderDistanceSettings::default());
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RenderDistanceSettings
{
	pub render_distance: f32,
}

impl RenderDistanceSettings
{
	pub fn new(distance: f32) -> Self
	{
		return Self {
			render_distance: distance,
		};
	}
}

impl Default for RenderDistanceSettings
{
	fn default() -> Self
	{
		Self::new(500.)
	}
}

#[derive(Component, FromTemplate, Clone)]
pub struct RenderDistanceVisibility
{
	pub offset: Vec3,
}

impl RenderDistanceVisibility
{
	pub fn chunk_centered() -> Self
	{
		Self {
			offset: Vec3::new(
				(Chunk::SIZE / 2) as f32 * SHORT_DIAGONAL,
				0.,
				(Chunk::SIZE / 2) as f32 * 1.5,
			),
		}
	}
}

impl Default for RenderDistanceVisibility
{
	fn default() -> Self
	{
		Self { offset: Vec3::ZERO }
	}
}

fn render_distance_system(
	mut objects: Query<(&Transform, &mut Visibility, &RenderDistanceVisibility)>,
	camera: Single<&Transform, With<MainCamera>>,
	settings: Res<RenderDistanceSettings>,
)
{
	let cam_pos = Vec3::new(camera.translation.x, 0.0, camera.translation.z);
	for (t, mut vis, r) in objects.iter_mut() {
		let dist = (cam_pos - (t.translation + r.offset)).length();
		if settings.render_distance < dist {
			*vis = Visibility::Hidden;
		} else {
			*vis = Visibility::Visible;
		}
	}
}
