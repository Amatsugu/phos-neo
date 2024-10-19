use crate::hex_utils::HexCoord;
use crate::{hex_utils::offset3d_to_world, prelude::*};
#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::math::VectorSpace;
use bevy::{
	prelude::*,
	render::{
		mesh::{Indices, PrimitiveTopology},
		render_asset::RenderAssetUsages,
	},
};

pub fn generate_chunk_mesh(chunk: &MeshChunkData) -> Mesh {
	#[cfg(feature = "tracing")]
	let span = info_span!("generate_chunk_mesh").entered();

	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE * 6;
	let mut verts = Vec::with_capacity(vertex_count);
	let mut uvs = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);
	let mut normals = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let idx = x + z * Chunk::SIZE;
			let height = chunk.heights[idx];
			let off_pos = Vec3::new(x as f32, height, z as f32);
			let tile_pos = offset3d_to_world(off_pos);
			let coord = HexCoord::from_grid_pos(x, z);
			let n = chunk.get_neighbors(&coord);

			create_tile(
				tile_pos,
				&n,
				&mut verts,
				&mut uvs,
				&mut indices,
				&mut normals,
				// &mut tex,
				chunk.textures[idx][0],
				chunk.textures[idx][1],
			);
		}
	}

	let mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
	.with_inserted_indices(Indices::U32(indices));
	return mesh;
}

fn create_tile(
	pos: Vec3,
	neighbors: &[f32; 6],
	verts: &mut Vec<Vec3>,
	uvs: &mut Vec<Vec2>,
	indices: &mut Vec<u32>,
	normals: &mut Vec<Vec3>,
	texture_index: u32,
	side_texture_index: u32,
) {
	let uv_offset = Vec2::splat(0.5);
	let tex_off = Vec2::new(texture_index as f32, 0.);
	let side_tex_off = Vec2::new(side_texture_index as f32, 0.);

	let idx = verts.len() as u32;
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
		let uv = (HEX_CORNERS[i].xz() / 2.) + uv_offset;
		uvs.push((uv / TEX_MULTI) + tex_off);
		normals.push(Vec3::Y);
	}
	for i in 0..3 {
		let off = i * 2;
		indices.push(off + idx);
		indices.push(((off + 1) % 6) + idx);
		indices.push(((off + 2) % 6) + idx);
	}
	indices.push(idx);
	indices.push(idx + 2);
	indices.push(idx + 4);

	for i in 0..neighbors.len() {
		let n_height = neighbors[i];
		if n_height < pos.y {
			create_tile_wall(pos, i, n_height, verts, uvs, indices, normals, side_tex_off);
		}
	}
}

pub fn generate_chunk_water_mesh(chunk: &MeshChunkData, sealevel: f32, map_width: usize, map_height: usize) -> Mesh {
	#[cfg(feature = "tracing")]
	let _gen_mesh = info_span!("Generate Water Surface Mesh").entered();
	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE * 7;
	let mut verts = Vec::with_capacity(vertex_count);
	let mut uvs = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);
	let mut normals = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let idx = x + z * Chunk::SIZE;
			let height = chunk.heights[idx];
			if height > sealevel {
				continue;
			}
			let off_pos = Vec3::new(x as f32, sealevel, z as f32);
			let tile_pos = offset3d_to_world(off_pos);
			let coord = HexCoord::from_grid_pos(x, z);
			let n = chunk.get_neighbors(&coord);

			create_tile_water_surface(tile_pos, &n, &mut verts, &mut uvs, &mut indices, &mut normals);
		}
	}
	let mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
	.with_inserted_indices(Indices::U32(indices));
	return mesh;
}

fn create_tile_water_surface(
	pos: Vec3,
	neighbors: &[f32; 6],
	verts: &mut Vec<Vec3>,
	uvs: &mut Vec<Vec2>,
	indices: &mut Vec<u32>,
	normals: &mut Vec<Vec3>,
) {
	let idx = verts.len() as u32;
	//todo: only use triangle fan when on shoreline
	verts.push(pos);
	uvs.push(Vec2::ZERO);
	normals.push(Vec3::Y);
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
		let mut uv = Vec2::ZERO;
		let n = neighbors[i];
		let nn = neighbors[(i + 5) % 6];

		if nn > pos.y || n > pos.y {
			uv = Vec2::ONE;
		}

		indices.push(idx);
		indices.push(idx + 1 + i as u32);
		indices.push(idx + 1 + ((i as u32 + 1) % 6));

		uvs.push(uv);
		normals.push(Vec3::Y);
	}
}

fn create_tile_wall(
	pos: Vec3,
	dir: usize,
	height: f32,
	verts: &mut Vec<Vec3>,
	uvs: &mut Vec<Vec2>,
	indices: &mut Vec<u32>,
	normals: &mut Vec<Vec3>,
	tex_off: Vec2,
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

	let n = HEX_NORMALS[dir];
	normals.push(n);
	normals.push(n);
	normals.push(n);
	normals.push(n);

	indices.push(idx);
	indices.push(idx + 2);
	indices.push(idx + 1);

	indices.push(idx + 1);
	indices.push(idx + 2);
	indices.push(idx + 3);

	uvs.push(Vec2::ZERO + tex_off);
	uvs.push((Vec2::new(1., 0.) / TEX_MULTI) + tex_off);
	uvs.push((Vec2::new(0., pos.y - height) / TEX_MULTI) + tex_off);
	uvs.push((Vec2::new(1., pos.y - height) / TEX_MULTI) + tex_off);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn generate_tile_wall() {
		let mut verts = Vec::new();
		let mut uvs = Vec::new();
		let mut normals = Vec::new();
		let mut indices = Vec::new();

		create_tile_wall(
			Vec3::ZERO,
			3,
			5.0,
			&mut verts,
			&mut uvs,
			&mut indices,
			&mut normals,
			Vec2::new(3.0, 0.0),
		);

		assert!(verts.len() == 4, "Number of verts don't match");
		assert!(uvs.len() == 4, "Number of uvs don't match");
		assert!(normals.len() == 4, "Number of normals don't match");
		assert!(indices.len() == 6, "Number of normals don't match");

		let index = uvs[0].x.floor();
		assert!(index == 3.0, "Texture Index could not be decoded");
	}

	#[test]
	fn generate_tile() {
		let mut verts = Vec::new();
		let mut uvs = Vec::new();
		let mut normals = Vec::new();
		let mut indices = Vec::new();

		//4 side faces
		let nbors = [2.0, 2.0, 0.0, 0.0, 0.0, 0.0];

		create_tile(Vec3::Y, &nbors, &mut verts, &mut uvs, &mut indices, &mut normals, 3, 7);

		assert!(verts.len() == (6 + 4 * 4), "Number of verts don't match");
		assert!(uvs.len() == (6 + 4 * 4), "Number of uvs don't match");
		assert!(normals.len() == (6 + 4 * 4), "Number of normals don't match");
		//12 tris for surface, 6 tris per side
		assert!(indices.len() == (12 + 4 * 6), "Number of indicies don't match");

		let top_index = uvs[0].x.floor();
		assert!(top_index == 3.0, "Top Texture Index could not be decoded");
		let side_index = uvs[6].x.floor();
		assert!(side_index == 7.0, "Top Texture Index could not be decoded");
	}
}
