use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use world_generation::hex_utils::HexCoord;

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub struct ResourceIdentifier {
	pub id: u32,
	pub qty: u32,
}

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub struct UnitIdentifier(u32);

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub struct TileIdentifier(u32);
