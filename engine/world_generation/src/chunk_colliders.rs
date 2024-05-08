use crate::{hex_utils::*, prelude::*};
#[cfg(feature = "tracing")]
use bevy::log::*;
use bevy::prelude::*;

const CHUNK_TOTAL: usize = Chunk::SIZE * Chunk::SIZE;

pub fn generate_chunk_collider(chunk: &Chunk, map: &Map) -> (Vec<Vec3>, Vec<[u32; 3]>) {
	#[cfg(feature = "tracing")]
	let span = info_span!("generate_chunk_collider").entered();
	let vertex_count: usize = CHUNK_TOTAL * 6;
	let mut verts = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);
	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let height = chunk.heights[x + z * Chunk::SIZE];
			let coord =
				HexCoord::from_offset(IVec2::new(x as i32, z as i32) + (chunk.chunk_offset * Chunk::SIZE as i32));
			let neighbors = map.get_neighbors(&coord);
			let off_pos = Vec3::new(x as f32, height, z as f32);
			let tile_pos = offset3d_to_world(off_pos);
			create_tile_collider(tile_pos, &mut verts, &mut indices, &neighbors);
		}
	}
	return (verts, indices);
}

fn create_tile_collider(pos: Vec3, verts: &mut Vec<Vec3>, indices: &mut Vec<[u32; 3]>, neighbors: &[Option<f32>; 6]) {
	let idx = verts.len() as u32;
	for i in 0..6 {
		let p = pos + HEX_CORNERS[i];
		verts.push(p);
	}

	//Top Surfave
	indices.push([idx, idx + 1, idx + 5]);
	indices.push([idx + 1, idx + 2, idx + 5]);
	indices.push([idx + 2, idx + 4, idx + 5]);
	indices.push([idx + 2, idx + 3, idx + 4]);

	for i in 0..neighbors.len() {
		let cur_n = neighbors[i];
		match cur_n {
			Some(n_height) => {
				if n_height < pos.y {
					create_tile_wall_collider(
						idx,
						Vec3::new(pos.x, n_height.min(pos.y - OUTER_RADIUS / 2.), pos.z),
						i,
						verts,
						indices,
					);
				}
			}
			_ => {}
		}
	}
}

fn create_tile_wall_collider(idx: u32, pos: Vec3, dir: usize, verts: &mut Vec<Vec3>, indices: &mut Vec<[u32; 3]>) {
	let idx2 = verts.len() as u32;

	verts.push(pos + HEX_CORNERS[dir]);
	verts.push(pos + HEX_CORNERS[(dir + 1) % 6]);

	let off = dir as u32;
	indices.push([idx + off, idx + ((off + 1) % 6), idx2 + 1]);
	indices.push([idx + off, idx2 + 1, idx2]);
}
