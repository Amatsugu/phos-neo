use bevy::prelude::*;
use quadtree_rs::{point::Point, Quadtree};
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
	tree: Quadtree<usize, UnitEntity>,
}

impl UnitSpacialSet {
	pub fn new(map_size: f32) -> Self {
		let n = f32::log2(map_size) / f32::log2(2.0);
		return Self {
			tree: Quadtree::new(n.ceil() as usize),
		};
	}

	pub fn add_unit(&mut self, unit: UnitEntity, pos: Vec3) -> Option<u64> {
		let p = pos.xz().as_uvec2();
		return self.tree.insert_pt(
			Point {
				x: p.x as usize,
				y: p.y as usize,
			},
			unit,
		);
	}

	pub fn move_unit(&mut self, handle: u64) {
		todo!();
	}
}
