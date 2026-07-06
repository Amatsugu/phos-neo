use avian3d::{
	collision::collider::{Collider, CollisionMargin},
	dynamics::{ccd::SpeculativeMargin, rigid_body::RigidBody},
};
use bevy::{light::NotShadowCaster, prelude::*};

use crate::{map_rendering::render_distance_system::RenderDistanceVisibility, prelude::PhosChunk};

pub struct ChunkPrefab;
impl ChunkPrefab
{
	pub fn terrain(
		translation: Vec3,
		mesh: Handle<Mesh>,
		material: Handle<impl Material>,
		collider: Collider,
		index: usize,
	) -> impl Bundle
	{
		(
			Mesh3d(mesh),
			MeshMaterial3d(material),
			Name::new(format!("Chunk {}", index)),
			Transform::from_translation(translation),
			PhosChunk::new(index),
			RenderDistanceVisibility::chunk_centered(),
			SpeculativeMargin(0.5),
			RigidBody::Static,
			CollisionMargin(0.1),
			collider,
		)
	}

	pub fn water(translation: Vec3, mesh: Handle<Mesh>, material: Handle<impl Material>, index: usize) -> impl Bundle
	{
		(
			Mesh3d(mesh),
			MeshMaterial3d(material),
			Transform::from_translation(translation),
			Name::new(format!("Water {}", index)),
			PhosChunk::new(index),
			NotShadowCaster,
		)
	}
}
