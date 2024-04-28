use crate::biome_painter::BiomePainterAsset;
use crate::hex_utils::HexCoord;
use crate::tile_manager::TileAsset;
use crate::tile_mapper::TileMapperAsset;
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

const HEX_NORMALS: [Vec3; 6] = [
	Vec3::new(
		INNER_RADIUS / 2.,
		0.,
		(OUTER_RADIUS + 0.5 * OUTER_RADIUS) / 2.,
	),
	Vec3::Z,
	Vec3::new(
		INNER_RADIUS / -2.,
		0.,
		(OUTER_RADIUS + 0.5 * OUTER_RADIUS) / 2.,
	),
	Vec3::new(
		INNER_RADIUS / -2.,
		0.,
		(OUTER_RADIUS + 0.5 * OUTER_RADIUS) / -2.,
	),
	Vec3::NEG_Z,
	Vec3::new(
		INNER_RADIUS / 2.,
		0.,
		(OUTER_RADIUS + 0.5 * OUTER_RADIUS) / -2.,
	),
];

pub fn generate_chunk_mesh(
	chunk: &Chunk,
	map: &Map,
	painter: &BiomePainterAsset,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) -> Mesh {
	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE * 6;
	let mut verts = Vec::with_capacity(vertex_count);
	let mut uvs = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);
	let mut normals = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let height = chunk.heights[x + z * Chunk::SIZE];
			let moisture = chunk.moisture[x + z * Chunk::SIZE];
			let temperature = chunk.temperature[x + z * Chunk::SIZE];
			let off_pos = Vec3::new(x as f32, height, z as f32);
			let tile_pos = offset3d_to_world(off_pos);
			let coord = HexCoord::from_offset(
				IVec2::new(x as i32, z as i32) + (chunk.chunk_offset * Chunk::SIZE as i32),
			);
			let n = map.get_neighbors(&coord);
			let biome = mappers.get(painter.sample_biome(moisture, temperature));
			let tile_handle = biome.unwrap().sample_tile(height);
			let tile = tiles.get(tile_handle).unwrap();

			create_tile(
				tile_pos,
				&n,
				&mut verts,
				&mut uvs,
				&mut indices,
				&mut normals,
				// &mut tex,
				tile.texture_id,
				tile.side_texture_id,
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

pub fn generate_packed_chunk_mesh(
	chunk: &Chunk,
	map: &Map,
	painter: &BiomePainterAsset,
	tiles: &Res<Assets<TileAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) -> Mesh {
	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE * 6;
	let mut packed_data = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);
	let mut heights = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let height = chunk.heights[x + z * Chunk::SIZE];
			let moisture = chunk.moisture[x + z * Chunk::SIZE];
			let temperature = chunk.temperature[x + z * Chunk::SIZE];
			let coord = HexCoord::from_offset(
				IVec2::new(x as i32, z as i32) + (chunk.chunk_offset * Chunk::SIZE as i32),
			);
			let n = map.get_neighbors(&coord);
			let biome = mappers.get(painter.sample_biome(moisture, temperature));
			let tile_handle = biome.unwrap().sample_tile(height);
			let tile = tiles.get(tile_handle).unwrap();

			create_packed_tile(
				UVec2::new(x as u32, z as u32),
				height,
				&n,
				&mut packed_data,
				&mut indices,
				&mut heights,
				tile.texture_id,
				tile.side_texture_id,
			);
		}
	}

	let mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
	)
	.with_inserted_attribute(ATTRIBUTE_PACKED_VERTEX_DATA, packed_data)
	.with_inserted_attribute(ATTRIBUTE_VERTEX_HEIGHT, heights)
	.with_inserted_indices(Indices::U32(indices));
	return mesh;
}

const TEX_MULTI: Vec2 = Vec2::new(1000., 1.);

fn create_packed_tile(
	offset: UVec2,
	height: f32,
	neighbors: &[Option<f32>; 6],
	packed_data: &mut Vec<u32>,
	indices: &mut Vec<u32>,
	heights: &mut Vec<f32>,
	texture_index: u32,
	side_texture_index: u32,
) {
	let idx = packed_data.len() as u32;

	packed_data.push(pack_vertex_data(offset, 0, texture_index));
	heights.push(height);
	for i in 0..6 {
		packed_data.push(pack_vertex_data(offset, i + 1, texture_index));
		indices.push(idx);
		indices.push(idx + 1 + i as u32);
		indices.push(idx + 1 + ((i as u32 + 1) % 6));
		heights.push(height);
	}

	for i in 0..neighbors.len() {
		let cur_n = neighbors[i];
		match cur_n {
			Some(n_height) => {
				if n_height < height {
					create_packed_tile_wall(
						offset,
						height,
						n_height,
						i,
						packed_data,
						indices,
						heights,
						side_texture_index,
					);
				}
			}
			_ => {}
		}
	}
}

fn create_packed_tile_wall(
	offset: UVec2,
	height_top: f32,
	height_bottom: f32,
	side: usize,
	packed_data: &mut Vec<u32>,
	indices: &mut Vec<u32>,
	heights: &mut Vec<f32>,
	side_texture_index: u32,
) {
	let idx = packed_data.len() as u32;

	let side_2 = ((side + 1) % 6) + 1;
	packed_data.push(pack_vertex_data(offset, side + 1, side_texture_index));
	packed_data.push(pack_vertex_data(offset, side_2, side_texture_index));
	packed_data.push(pack_vertex_data(offset, side + 1, side_texture_index));
	packed_data.push(pack_vertex_data(offset, side_2, side_texture_index));

	heights.push(height_top);
	heights.push(height_top);
	heights.push(height_bottom);
	heights.push(height_bottom);

	indices.push(idx);
	indices.push(idx + 2);
	indices.push(idx + 1);

	indices.push(idx + 1);
	indices.push(idx + 2);
	indices.push(idx + 3);
}

fn create_tile(
	pos: Vec3,
	neighbors: &[Option<f32>; 6],
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
	uvs.push((uv_offset / TEX_MULTI) + tex_off);
	verts.push(pos);
	normals.push(Vec3::Y);
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
		let uv = (HEX_CORNERS[i].xz() / 2.) + uv_offset;
		uvs.push((uv / TEX_MULTI) + tex_off);
		indices.push(idx);
		indices.push(idx + 1 + i as u32);
		indices.push(idx + 1 + ((i as u32 + 1) % 6));
		normals.push(Vec3::Y);
	}

	for i in 0..neighbors.len() {
		let cur_n = neighbors[i];
		match cur_n {
			Some(n_height) => {
				if n_height < pos.y {
					create_tile_wall(pos, i, n_height, verts, uvs, indices, normals, side_tex_off);
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

fn pack_vertex_data(offset: UVec2, vert: usize, tex: u32) -> u32 {
	//6 + 6 bits offset
	//4 bits vert
	//12 bits texture
	let mut data = offset.x;
	data += (offset.y) << 6;
	data += (vert as u32) << (6 + 6);
	data += tex << (6 + 6 + 4);

	return data;
}
