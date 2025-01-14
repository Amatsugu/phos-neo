// use bevy::{
// 	prelude::*,
// 	render::{
// 		render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
// 		view::RenderLayers,
// 	},
// 	window::PrimaryWindow,
// };
// use bevy_lunex::prelude::*;
// use shared::tags::MainCamera;

// pub struct LunexSetupPlugin;

// impl Plugin for LunexSetupPlugin {
// 	fn build(&self, app: &mut App) {
// 		app.add_plugins(UiDefaultPlugins);

// 		#[cfg(debug_assertions)]
// 		app.add_plugins(UiDebugPlugin::<MainUi>::new());
// 		app.add_systems(PostStartup, setup_cameras);
// 	}
// }

// fn setup_cameras(
// 	mut commands: Commands,
// 	assets: Res<AssetServer>,
// 	mut main_3d_camera_q: Query<&mut Camera, With<MainCamera>>,
// 	window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
// 	//Prepare Render Texture
// 	let win = window_query.single();
// 	let size = Extent3d {
// 		width: win.physical_width(),
// 		height: win.physical_height(),
// 		..default()
// 	};

// 	let mut image = Image {
// 		texture_descriptor: TextureDescriptor {
// 			label: None,
// 			size,
// 			dimension: TextureDimension::D2,
// 			format: TextureFormat::Bgra8UnormSrgb,
// 			mip_level_count: 1,
// 			sample_count: 1,
// 			usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
// 			view_formats: &[],
// 		},
// 		..default()
// 	};
// 	image.resize(size);

// 	//Configure 3D Camera
// 	let mut cam = main_3d_camera_q.single_mut();
// 	let render_image = assets.add(image);
// 	cam.target = render_image.clone().into();
// 	cam.order = -1;
// 	cam.clear_color = ClearColorConfig::Custom(LinearRgba::NONE.into());

// 	//Add Render Texture image
// 	commands
// 		.spawn((
// 			UiTreeBundle::<MainUi>::from(UiTree::new2d("Main UI")),
// 			SourceFromCamera,
// 			RenderLayers::layer(1),
// 		))
// 		.with_children(|ui| {
// 			ui.spawn((
// 				UiLink::<MainUi>::path("Root"),
// 				UiLayout::window_full().size((Rl(100.), Rl(100.))).pack::<Base>(),
// 				RenderLayers::layer(1),
// 			));
// 			ui.spawn((
// 				UiLink::<MainUi>::path("Root/Camera3D"),
// 				UiLayout::solid()
// 					.size((Rl(100.), Rl(100.)))
// 					.scaling(Scaling::Fill)
// 					.pack::<Base>(),
// 				UiImage2dBundle::from(render_image),
// 				PickingPortal,
// 				RenderLayers::layer(1),
// 			));
// 		});

// 	//Spawn 2d UI Camera
// 	commands.spawn((
// 		MainUi,
// 		Camera2dBundle {
// 			transform: Transform::from_xyz(0.0, 0.0, 1000.0),
// 			..default()
// 		},
// 		RenderLayers::from_layers(&[1]),
// 	));
// }
