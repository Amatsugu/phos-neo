use asset_loader::create_asset_loader;
use bevy::{gltf::GltfMesh, prelude::*};
use serde::{Deserialize, Serialize};
use shared::{identifiers::ResourceIdentifier, prefab_defination::*};

use crate::{
	buildings::{
		conduit_building::ResourceConduitInfo, factory_building::FactoryBuildingInfo,
		resource_gathering::ResourceGatheringBuildingInfo,
	},
	footprint::BuildingFootprint,
	prelude::Building,
};

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct BuildingAsset {
	pub name: String,
	pub description: String,
	pub footprint: BuildingFootprint,
	pub prefab_path: String,
	#[serde(skip)]
	pub prefab: Handle<Gltf>,
	pub base_mesh_path: String,

	pub cost: Vec<ResourceIdentifier>,
	pub consumption: Vec<ResourceIdentifier>,
	pub production: Vec<ResourceIdentifier>,

	pub health: u32,

	pub building_type: BuildingType,
	pub children: Option<Vec<PrefabDefination>>,
}

impl BuildingAsset {
	pub fn spawn(
		&self,
		pos: Vec3,
		rot: Quat,
		gltf: &Gltf,
		commands: &mut Commands,
		meshes: &Assets<GltfMesh>,
	) -> Option<Entity> {
		let keys: Vec<_> = gltf.named_meshes.keys().collect();
		info!("{keys:#?}");
		let mesh_handle = &gltf.named_meshes[&self.base_mesh_path.clone().into_boxed_str()];
		if let Some(gltf_mesh) = meshes.get(mesh_handle.id()) {
			let (mesh, mat) = gltf_mesh.unpack();
			let mut entity = commands.spawn((
				PbrBundle {
					mesh,
					material: mat,
					transform: Transform::from_translation(pos)
						.with_rotation(Quat::from_rotation_arc(Vec3::NEG_Z, Vec3::Y) * rot),
					..Default::default()
				},
				Building,
			));
			if let Some(children) = &self.children {
				entity.with_children(|b| {
					for child in children {
						child.spawn_recursive(gltf, b, meshes);
					}
				});
			}
			return Some(entity.id());
		}
		return None;
	}
}

#[derive(Serialize, Deserialize, Debug, TypePath)]
pub enum BuildingType {
	Basic,
	Gathering(ResourceGatheringBuildingInfo),
	FactoryBuildingInfo(FactoryBuildingInfo),
	ResourceConduit(ResourceConduitInfo),
}

create_asset_loader!(
	BuildingAssetPlugin,
	BuildingAssetLoader,
	BuildingAsset,
	&["building", "building.ron"],
	prefab_path -> prefab
	;?
);
