use bevy::{
	gltf::{Gltf, GltfMesh},
	math::{Quat, Vec3},
	prelude::*,
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PrefabDefination {
	pub path: String,
	pub pos: Vec3,
	pub rot: Vec3,
	pub children: Option<Vec<PrefabDefination>>,
	pub animations: Option<Vec<AnimationComponent>>,
}

impl PrefabDefination {
	pub fn spawn_recursive(&self, gltf: &Gltf, commands: &mut ChildBuilder, meshes: &Assets<GltfMesh>) {
		let mesh_handle = &gltf.named_meshes[&self.path.clone().into_boxed_str()];
		if let Some(gltf_mesh) = meshes.get(mesh_handle.id()) {
			let (m, mat) = gltf_mesh.unpack();
			let mut entity = commands.spawn(PbrBundle {
				mesh: m,
				material: mat,
				transform: Transform::from_translation(self.pos).with_rotation(Quat::from_euler(
					bevy::math::EulerRot::XYZ,
					self.rot.x,
					self.rot.y,
					self.rot.z,
				)),
				..Default::default()
			});
			if let Some(children) = &self.children {
				entity.with_children(|b| {
					for child in children {
						child.spawn_recursive(gltf, b, meshes);
					}
				});
			}
		}
	}
}

pub trait UnpackGltfMesh {
	fn unpack(&self) -> (Handle<Mesh>, Handle<StandardMaterial>);
}

impl UnpackGltfMesh for GltfMesh {
	fn unpack(&self) -> (Handle<Mesh>, Handle<StandardMaterial>) {
		let p = &self.primitives[0];
		let mut mat: Handle<StandardMaterial> = default();
		if let Some(mesh_material) = &p.material {
			mat = mesh_material.clone();
		}
		return (p.mesh.clone(), mat);
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnimationComponent {
	Rotation,
	Slider,
}
