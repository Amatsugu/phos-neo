use bevy::prelude::Entity;
use quadtree_rs::Quadtree;
use shared::tags::Faction;
use world_generation::hex_utils::HexCoord;

use crate::components::UnitDomain;

pub struct UnitEntity {
	pub entity: Entity,
	pub domain: UnitDomain,
	pub unitType: (),
	pub faction: Faction,
}

pub struct UnitSpacialSet {
	tree: Quadtree<i32, UnitEntity>,
}

impl UnitSpacialSet {
	pub fn new() {

	}
}
