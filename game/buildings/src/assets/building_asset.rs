use asset_loader::create_asset_loader;
use bevy::{
	gltf::{GltfMesh, GltfNode},
	prelude::*,
};
use serde::{Deserialize, Serialize};
use shared::{component_defination::ComponentDefination, identifiers::ResourceIdentifier, prefab_defination::*};

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
	pub components: Option<Vec<ComponentDefination>>,
}

impl BuildingAsset {
	pub fn spawn(
		&self,
		pos: Vec3,
		rot: Quat,
		gltf: &Gltf,
		commands: &mut Commands,
		meshes: &Assets<GltfMesh>,
		nodes: &Assets<GltfNode>,
	) -> Option<Entity> {
		let base_node = &gltf.named_nodes[&self.base_mesh_path.clone().into_boxed_str()];
		if let Some(node) = nodes.get(base_node.id()) {
			if let Some(mesh_handle) = &node.mesh {
				if let Some(gltf_mesh) = meshes.get(mesh_handle.id()) {
					let (mesh, mat) = gltf_mesh.unpack();
					let mut entity = commands.spawn((
						PbrBundle {
							mesh,
							material: mat,
							transform: Transform::from_translation(pos).with_rotation(rot),
							..Default::default()
						},
						Building,
					));
					entity.with_children(|b| {
						for child in &node.children {
							self.process_node(child, meshes, b, &node.name);
						}
					});
					if let Some(component) = self.get_component_def(&format!("/{0}", &node.name)) {
						component.apply(&mut entity);
					}
					return Some(entity.id());
				}
			}
		}
		return None;
	}

	fn process_node(
		&self,
		node: &GltfNode,
		meshes: &Assets<GltfMesh>,
		commands: &mut ChildBuilder,
		parent: &String,
	) -> Option<Entity> {
		let path = format!("{0}/{1}", parent, node.name);
		if let Some(mesh) = &node.mesh {
			if let Some(gltf_mesh) = meshes.get(mesh.id()) {
				let (mesh, mat) = gltf_mesh.unpack();
				let mut entity = commands.spawn((
					PbrBundle {
						mesh,
						material: mat,
						transform: node.transform,
						..Default::default()
					},
					Building,
				));
				entity.with_children(|b| {
					for child in &node.children {
						self.process_node(child, meshes, b, &path);
					}
				});
				if let Some(component) = self.get_component_def(&path) {
					component.apply(&mut entity);
				}
				return Some(entity.id());
			}
		}
		return None;
	}

	fn get_component_def(&self, path: &String) -> Option<&ComponentDefination> {
		if let Some(components) = &self.components {
			for c in components {
				if c.path.eq(path) {
					return Some(c);
				}
			}
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
