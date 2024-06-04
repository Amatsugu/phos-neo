use bevy::{
	prelude::*,
	render::{mesh::MeshVertexAttribute, render_resource::VertexFormat},
};

use crate::hex_utils::{INNER_RADIUS, OUTER_RADIUS};

pub const TEX_MULTI: Vec2 = Vec2::new(1000., 1.);

pub const HEX_CORNERS: [Vec3; 6] = [
	Vec3::new(0., 0., OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(0., 0., -OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
];

pub const HEX_NORMALS: [Vec3; 6] = [
	Vec3::new(INNER_RADIUS / 2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / 2.),
	Vec3::Z,
	Vec3::new(INNER_RADIUS / -2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / 2.),
	Vec3::new(INNER_RADIUS / -2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / -2.),
	Vec3::NEG_Z,
	Vec3::new(INNER_RADIUS / 2., 0., (OUTER_RADIUS + 0.5 * OUTER_RADIUS) / -2.),
];

pub const ATTRIBUTE_PACKED_VERTEX_DATA: MeshVertexAttribute =
	MeshVertexAttribute::new("PackedVertexData", 988540817, VertexFormat::Uint32);
pub const ATTRIBUTE_VERTEX_HEIGHT: MeshVertexAttribute =
	MeshVertexAttribute::new("VertexHeight", 988540717, VertexFormat::Float32);

pub const ATTRIBUTE_TEXTURE_INDEX: MeshVertexAttribute =
	MeshVertexAttribute::new("TextureIndex", 988540917, VertexFormat::Uint32);
