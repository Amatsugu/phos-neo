use bevy::{
	ecs::{relationship::RelatedSpawnerCommands, system::EntityCommands},
	gltf::{Gltf, GltfMesh},
	math::{Quat, Vec3},
	prelude::*,
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PrefabDefination
{
	pub path: String,
	pub pos: Vec3,
	pub rot: Vec3,
	pub children: Option<Vec<PrefabDefination>>,
	pub animations: Option<Vec<AnimationComponent>>,
}

impl PrefabDefination
{
	pub fn spawn_recursive(
		&self,
		gltf: &Gltf,
		commands: &mut RelatedSpawnerCommands<ChildOf>,
		meshes: &Assets<GltfMesh>,
	)
	{
		let mesh_handle = &gltf.named_meshes[&self.path.clone().into_boxed_str()];
		if let Some(gltf_mesh) = meshes.get(mesh_handle.id()) {
			if let Some(primitive) = gltf_mesh.primitives.first() {
				let mesh = primitive.mesh.clone();
				let mat = primitive
					.material
					.clone()
					.expect(format!("Mesh '{}' does not have a meterial", primitive.name.as_str()).as_str());
				let mut entity = commands.spawn((
					Mesh3d(mesh),
					MeshMaterial3d(mat),
					Transform::from_translation(self.pos).with_rotation(Quat::from_euler(
						bevy::math::EulerRot::XYZ,
						self.rot.x,
						self.rot.y,
						self.rot.z,
					)),
				));
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
}

pub trait UnpackGltfMesh
{
	fn unpack(&self) -> (Handle<Mesh>, Handle<StandardMaterial>);
}

impl UnpackGltfMesh for GltfMesh
{
	fn unpack(&self) -> (Handle<Mesh>, Handle<StandardMaterial>)
	{
		let p = &self.primitives[0];
		let mut mat: Handle<StandardMaterial> = default();
		if let Some(mesh_material) = &p.material {
			mat = mesh_material.clone();
		}
		return (p.mesh.clone(), mat);
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnimationComponent
{
	Rotation(RotationAnimation),
	Slider,
}

#[derive(Serialize, Deserialize, Debug, Component, Clone, Copy)]
pub struct RotationAnimation
{
	pub axis: Vec3,
	pub speed: f32,
}

impl AnimationComponent
{
	pub fn apply(&self, commands: &mut EntityCommands)
	{
		match self {
			AnimationComponent::Rotation(comp) => {
				commands.insert(comp.clone());
			}
			AnimationComponent::Slider => todo!(),
		};
	}
}
