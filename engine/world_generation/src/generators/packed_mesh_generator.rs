use crate::hex_utils::HexCoord;
use crate::map::biome_map::BiomeChunk;
use crate::prelude::*;
use crate::tile_manager::TileAsset;
use crate::tile_mapper::TileMapperAsset;
use crate::{biome_asset::BiomeAsset, biome_painter::BiomePainterAsset};
use bevy::{
	prelude::*,
	render::{
		mesh::{Indices, PrimitiveTopology},
		render_asset::RenderAssetUsages,
	},
};

pub fn generate_packed_chunk_mesh(
	chunk: &Chunk,
	map: &Map,
	biome_chunk: &BiomeChunk,
	painter: &BiomePainterAsset,
	tiles: &Res<Assets<TileAsset>>,
	biomes: &Res<Assets<BiomeAsset>>,
	mappers: &Res<Assets<TileMapperAsset>>,
) -> Mesh {
	let vertex_count: usize = Chunk::SIZE * Chunk::SIZE * 6;
	let mut packed_data = Vec::with_capacity(vertex_count);
	let mut indices = Vec::with_capacity(vertex_count);
	let mut heights = Vec::with_capacity(vertex_count);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let height = chunk.heights[x + z * Chunk::SIZE];
			let data = biome_chunk.data[x + z * Chunk::SIZE];
			let coord =
				HexCoord::from_offset(IVec2::new(x as i32, z as i32) + (chunk.chunk_offset * Chunk::SIZE as i32));
			let n = map.get_neighbors(&coord);
			let biome = biomes.get(painter.sample_biome(biomes, &data)).unwrap();

			let mapper = mappers.get(biome.tile_mapper.id());
			let tile_handle = mapper.unwrap().sample_tile(height);
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
