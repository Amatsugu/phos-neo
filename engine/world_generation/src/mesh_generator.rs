use crate::hex_utils::HexCoord;
use crate::{
	hex_utils::{offset3d_to_world, INNER_RADIUS, OUTER_RADIUS},
	prelude::*,
};
use bevy::{
	prelude::*,
	render::{
		mesh::{Indices, PrimitiveTopology},
		render_asset::RenderAssetUsages,
	},
};
use std::vec::Vec;

const HEX_CORNERS: [Vec3; 6] = [
	Vec3::new(0., 0., OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(0., 0., -OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
];

pub fn generate_chunk_mesh(chunk: &Chunk, map: &Map) -> Mesh {
	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE * 6;
	let mut verts = Vec::with_capacity(vertex_count);
	let mut uvs = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let height = chunk.points[x + z * Chunk::SIZE];
			let off_pos = Vec3::new(x as f32, height, z as f32);
			let tile_pos = offset3d_to_world(off_pos);
			let coord = HexCoord::from_offset(
				IVec2::new(x as i32, z as i32) + (chunk.chunk_offset * Chunk::SIZE as i32),
			);
			let n = map.get_neighbors(&coord);
			create_tile(
				tile_pos,
				&n,
				&mut verts,
				&mut uvs,
				&mut indices,
				((x + z * Chunk::SIZE) % 32) as u32,
			);
		}
	}

	let mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
	.with_inserted_indices(Indices::U32(indices))
	.with_duplicated_vertices()
	.with_computed_flat_normals();
	return mesh;
}

//TODO: figure out texture index
fn create_tile(
	pos: Vec3,
	neighbors: &[Option<f32>; 6],
	verts: &mut Vec<Vec3>,
	uvs: &mut Vec<Vec2>,
	indices: &mut Vec<u32>,
	texture_index: u32,
) {
	let tex_x = texture_index % 11;
	let tex_y = texture_index / 11;
	let x_min = tex_x as f32 / 11.;
	let y_min = tex_y as f32 / 8.;
	const TX_UNIT: Vec2 = Vec2::new(1. / 11., 1. / 8.);

	let uv_offset = Vec2::splat(0.5);

	let idx = verts.len() as u32;
	uvs.push(uv_offset);
	verts.push(pos);
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
		let uv = (HEX_CORNERS[i].xz() / 2.) + uv_offset;
		uvs.push(uv);
		indices.push(idx);
		indices.push(idx + 1 + i as u32);
		indices.push(idx + 1 + ((i as u32 + 1) % 6));
	}

	for i in 0..neighbors.len() {
		let cur_n = neighbors[i];
		match cur_n {
			Some(n_height) => {
				if n_height < pos.y {
					create_tile_wall(pos, i, n_height, verts, uvs, indices, Vec2::ZERO, Vec2::ONE);
				}
			}
			_ => {}
		}
	}
}

fn create_tile_wall(
	pos: Vec3,
	dir: usize,
	height: f32,
	verts: &mut Vec<Vec3>,
	uvs: &mut Vec<Vec2>,
	indices: &mut Vec<u32>,
	tx_min: Vec2,
	tx_max: Vec2,
) {
	let p1 = HEX_CORNERS[(dir) % 6] + pos;
	let p2 = HEX_CORNERS[(dir + 1) % 6] + pos;
	let p3 = Vec3::new(p1.x, height, p1.z);
	let p4 = Vec3::new(p2.x, height, p2.z);

	let idx = verts.len() as u32;

	verts.push(p1);
	verts.push(p2);
	verts.push(p3);
	verts.push(p4);

	indices.push(idx);
	indices.push(idx + 2);
	indices.push(idx + 1);

	indices.push(idx + 1);
	indices.push(idx + 2);
	indices.push(idx + 3);

	//TODO: scale texture based on height
	uvs.push(tx_min);
	uvs.push(Vec2::new(tx_max.x, tx_min.y));
	uvs.push(Vec2::new(tx_min.x, tx_max.y));
	uvs.push(tx_max);
}
