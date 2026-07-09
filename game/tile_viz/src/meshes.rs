use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*};
use hex::{INNER_RADIUS, OUTER_RADIUS};

const TILTE_VIZ_OFFSET: f32 = 0.1;

const HEX_CORNERS_VIZ: [Vec3; 6] = [
	Vec3::new(0., TILTE_VIZ_OFFSET, OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, TILTE_VIZ_OFFSET, 0.5 * OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, TILTE_VIZ_OFFSET, -0.5 * OUTER_RADIUS),
	Vec3::new(0., TILTE_VIZ_OFFSET, -OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, TILTE_VIZ_OFFSET, -0.5 * OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, TILTE_VIZ_OFFSET, 0.5 * OUTER_RADIUS),
];

pub(crate) fn get_tile_surface_mesh() -> Mesh
{
	let uv_offset = Vec2::splat(0.5);
	let mut verts = Vec::with_capacity(6);
	let mut uvs = Vec::with_capacity(6);
	let mut normals = Vec::with_capacity(6);
	let mut indices = Vec::with_capacity(12);

	for corner in HEX_CORNERS_VIZ {
		verts.push(corner);
		let uv = (corner.xz() / 2.) + uv_offset;
		uvs.push(uv);
		normals.push(Vec3::Y);
	}
	for i in 0..3 {
		let off = i * 2;
		indices.push(off);
		indices.push((off + 1) % 6);
		indices.push((off + 2) % 6);
	}
	indices.push(0);
	indices.push(2);
	indices.push(4);

	Mesh::new(
		bevy::mesh::PrimitiveTopology::TriangleList,
		RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
	.with_inserted_indices(Indices::U32(indices))
}
