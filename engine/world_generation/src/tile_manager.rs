use std::{collections::HashMap, fs};

use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct TileManager {
	pub tiles: HashMap<u32, TileDefination>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileDefination {
	pub id: u32,
	pub texture_id: u32,
	pub side_texture_id: u32,
}

impl TileManager {
	pub fn new(path: String) -> Option<Self> {
		let mut cur_id: u32 = 0;

		let paths = fs::read_dir(path);

		if paths.is_err() {
			print!("{}", paths.err().unwrap());
			return None;
		}

		let mut manager = TileManager {
			tiles: HashMap::new(),
		};

		for dir_entry in paths.unwrap() {
			let cur_path = dir_entry.unwrap().path();
			if !cur_path.is_file() {
				continue;
			}
			let data = fs::read_to_string(cur_path);
			if data.is_err() {
				print!("{}", data.err().unwrap());
				return None;
			}

			let result: Result<TileDefination, serde_json::Error> =
				serde_json::from_str(data.unwrap().as_str());

			if result.is_err() {
				print!("{}", result.err().unwrap());
				return None;
			}

			let mut tile = result.unwrap();
			tile.id = cur_id;

			manager.tiles.insert(tile.id, tile);
			cur_id += 1;
		}

		return Some(manager);
	}
}
