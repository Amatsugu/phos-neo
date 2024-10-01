use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use world_generation::hex_utils::HexCoord;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceIdentifier {
	pub id: u32,
	pub qty: u32,
}
