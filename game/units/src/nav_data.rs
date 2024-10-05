use ordered_float::OrderedFloat;
use world_generation::{
	hex_utils::HexCoord,
	prelude::{Chunk, Map},
};

#[derive(Clone)]
pub struct NavData {
	pub tiles: Vec<NavTile>,
	pub map_height: usize,
	pub map_width: usize,
}

impl NavData {
	pub fn get_neighbors(&self, coord: &HexCoord) -> Vec<(HexCoord, OrderedFloat<f32>)> {
		let mut neighbors = Vec::with_capacity(6);
		for i in 0..6 {
			let n = coord.get_neighbor(i);
			if !self.is_in_bounds(&n) {
				continue;
			}
			neighbors.push((n, OrderedFloat(1.0)));
		}
		return neighbors;
	}
	pub fn get(&self, coord: &HexCoord) -> &NavTile {
		return &self.tiles[coord.to_index(self.map_width)];
	}

	pub fn is_in_bounds(&self, pos: &HexCoord) -> bool {
		return pos.is_in_bounds(self.map_height, self.map_width);
	}

	pub fn build(map: &Map) -> NavData {
		let mut tiles = Vec::with_capacity(map.get_tile_count());
		let h = map.get_tile_height();
		let w = map.get_tile_width();
		for y in 0..h {
			for x in 0..w {
				let coord = HexCoord::from_grid_pos(x, y);
				let height = map.sample_height(&coord);
				let tile = NavTile {
					coord,
					height,
					move_cost: 1.0,
				};
				tiles.push(tile);
			}
		}

		return NavData {
			tiles,
			map_width: w,
			map_height: h,
		};
	}
}

#[derive(Clone)]
pub struct NavTile {
	pub height: f32,
	pub move_cost: f32,
	pub coord: HexCoord,
}

impl NavTile {
	pub fn calculate_heuristic(&self, to: &HexCoord) -> OrderedFloat<f32> {
		todo!();
	}
}
