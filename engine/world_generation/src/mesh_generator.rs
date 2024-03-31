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

const HEX_CORNERS: [Vec3; 6] = [
	Vec3::new(0., 0., OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(0., 0., -OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
];

pub fn generate_chunk_mesh(chunk: &Chunk, map: &Map) -> Mesh {
	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE;
	let mut verts = Vec::with_capacity(vertex_count);
	let mut uvs = Vec::with_capacity(vertex_count);
	let mut normals = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let coord = HexCoord::from_grid_pos(x, z);
			let height = chunk.points[coord.to_index(Chunk::SIZE) as usize];
			let off_pos = Vec3::new(x as f32, height, z as f32);
			let grid_pos = offset3d_to_world(off_pos);
			let n = map.get_neighbors(&HexCoord::from_offset(
				coord.to_offset() + chunk.chunk_offset,
			));
			create_tile(
				grid_pos,
				&n,
				&mut verts,
				&mut uvs,
				&mut normals,
				&mut indices,
				0,
			);
		}
	}

	let mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
	// .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
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
	normals: &mut Vec<Vec3>,
	indices: &mut Vec<u32>,
	texture_index: u32,
) {
	let idx = verts.len() as u32;
	let center = Vec3::new(pos.x, 0., pos.z);
	normals.push(Vec3::Y);
	uvs.push(pos.xz());
	verts.push(pos);
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
		uvs.push(p.xz());
		normals.push((p - center).normalize());
		indices.push(idx);
		indices.push(idx + 1 + i as u32);
		indices.push(idx + 1 + ((i as u32 + 1) % 6));
	}

	for i in 0..neighbors.len() {
		let cur_n = neighbors[i];
		match cur_n {
			Some(n_height) => {
				if true {
					create_tile_wall(
						pos,
						i,
						pos.y + 1.,
						verts,
						uvs,
						normals,
						indices,
						texture_index,
					);
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
	normals: &mut Vec<Vec3>,
	indices: &mut Vec<u32>,
	_texture_index: u32,
) {
	println!("{dir}");
	let p1 = HEX_CORNERS[(dir + 1) % 6] + pos;
	let p2 = HEX_CORNERS[(dir + 2) % 6] + pos;
	let p3 = Vec3::new(p1.x, height, p1.z);
	let p4 = Vec3::new(p2.x, height, p2.z);

	let normal = Vec3::Y;

	let idx = verts.len() as u32;

	verts.push(p1);
	verts.push(p2);
	verts.push(p3);
	verts.push(p4);

	normals.push(normal);
	normals.push(normal);
	normals.push(normal);
	normals.push(normal);

	indices.push(idx);
	indices.push(idx + 2);
	indices.push(idx + 1);

	indices.push(idx + 1);
	indices.push(idx + 2);
	indices.push(idx + 3);

	uvs.push(Vec2::new(0., 0.));
	uvs.push(Vec2::new(1., 0.));
	uvs.push(Vec2::new(0., 1.));
	uvs.push(Vec2::new(1., 1.));
}
