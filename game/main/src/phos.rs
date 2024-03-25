use bevy::{pbr::CascadeShadowConfig, prelude::*};

pub struct PhosGamePlugin;

impl Plugin for PhosGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_game);
    }
}

fn init_game(mut commands: Commands) {
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0., 50., 0.)
				.looking_at(Vec3::new(50., 0., 50.), Vec3::Y),
			..default()
		},
	));

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: false,
			..default()
		},
		cascade_shadow_config: CascadeShadowConfig {
			bounds: vec![20., 40., 80., 1000., 5000., 19000., 20000.],
			..default()
		},
		transform: Transform::from_xyz(0.0, 16.0, 5.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
