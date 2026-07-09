use bevy::prelude::*;
use shared::states::AssetLoadState;
use world_generation::mapping::map::Map;

use crate::{
	components::{HexAreaSelection, TileSurfaceVisualization},
	meshes::get_tile_surface_mesh,
	resources::{TileSurfaceMaterials, VisualzationMeshes},
};

pub struct TileVizPlugin;

impl Plugin for TileVizPlugin
{
	fn build(&self, app: &mut App)
	{
		app.init_resource::<TileSurfaceMaterials>();
		app.add_systems(
			Update,
			(setup_meshes, setup_materials).run_if(in_state(AssetLoadState::FinalizeAssets)),
		);
		app.add_observer(spawn_hex_area);
	}
}

fn setup_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>)
{
	let handle = meshes.add(get_tile_surface_mesh());

	commands.insert_resource(VisualzationMeshes { tile_surface: handle });
	info!("add mesh");
}
fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>)
{
	commands.insert_resource(TileSurfaceMaterials {
		selection_material: materials.add(StandardMaterial {
			base_color: Color::WHITE,
			alpha_mode: AlphaMode::Add,
			unlit: true,
			..Default::default()
		}),
		..default()
	});
}

fn spawn_hex_area(
	added: On<Insert, HexAreaSelection>,
	mut commands: Commands,
	entities: Query<(&HexAreaSelection, Option<&Children>)>,
	map: Res<Map>,
	meshes: Res<VisualzationMeshes>,
	materials: Res<TileSurfaceMaterials>,
)
{
	if let Ok((selection, children)) = entities.get(added.entity) {
		#[cfg(feature = "tracing")]
		let _ = info_span!("Hex Area Visualization").entered();
		if let Some(children) = children {
			for e in children {
				commands.entity(*e).despawn();
			}
		}
		let entities = map.hex_select(&selection.0, selection.1, true, |c, h, _i| {
			tile_surface(
				c.to_world(h.max(map.sealevel)),
				meshes.tile_surface.clone(),
				materials.selection_material.clone(),
			)
		});

		commands.entity(added.entity).with_children(|c| {
			for bundle in entities {
				c.spawn(bundle);
			}
		});
	}
}

fn tile_surface<M: Material>(translation: Vec3, mesh: Handle<Mesh>, material: Handle<M>) -> impl Bundle
{
	(
		Mesh3d(mesh),
		MeshMaterial3d(material),
		Transform::from_translation(translation),
		TileSurfaceVisualization,
	)
}
