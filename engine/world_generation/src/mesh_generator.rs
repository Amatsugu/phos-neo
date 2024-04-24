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
	Vec3::new(0., 0., OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
	Vec3::new(INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(0., 0., -OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
	Vec3::new(-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
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
	let mut texture_indicies = Vec::with_capacity(vertex_count);

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
				&mut texture_indicies,
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
	.with_inserted_attribute(ATTRIBUTE_TEXTURE_INDEX, texture_indicies)
	.with_inserted_indices(Indices::U32(indices))
	.with_duplicated_vertices()
	.with_computed_flat_normals();
	return mesh;
}

fn create_tile(
	pos: Vec3,
	neighbors: &[Option<f32>; 6],
	verts: &mut Vec<Vec3>,
	uvs: &mut Vec<Vec2>,
	indices: &mut Vec<u32>,
	texture_indices: &mut Vec<u32>,
	texture_index: u32,
	side_texture_index: u32,
) {
	let uv_offset = Vec2::splat(0.5);

	let idx = verts.len() as u32;
	uvs.push(uv_offset);
	texture_indices.push(texture_index);
	verts.push(pos);
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
		let uv = (HEX_CORNERS[i].xz() / 2.) + uv_offset;
		uvs.push(uv);
		indices.push(idx);
		indices.push(idx + 1 + i as u32);
		indices.push(idx + 1 + ((i as u32 + 1) % 6));
		texture_indices.push(texture_index);
	}

	for i in 0..neighbors.len() {
		let cur_n = neighbors[i];
		match cur_n {
			Some(n_height) => {
				if n_height < pos.y {
					create_tile_wall(
						pos,
						i,
						n_height,
						verts,
						uvs,
						indices,
						texture_indices,
						side_texture_index,
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
	indices: &mut Vec<u32>,
	texture_indices: &mut Vec<u32>,
	texture_index: u32,
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

	texture_indices.push(texture_index);
	texture_indices.push(texture_index);
	texture_indices.push(texture_index);
	texture_indices.push(texture_index);

	indices.push(idx);
	indices.push(idx + 2);
	indices.push(idx + 1);

	indices.push(idx + 1);
	indices.push(idx + 2);
	indices.push(idx + 3);

	uvs.push(Vec2::ZERO);
	uvs.push(Vec2::new(1., 0.));
	uvs.push(Vec2::new(0., pos.y - height));
	uvs.push(Vec2::new(1., pos.y - height));
}
