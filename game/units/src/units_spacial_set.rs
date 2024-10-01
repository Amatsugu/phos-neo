use bevy::prelude::*;
use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};
use shared::tags::Faction;

use crate::{components::UnitDomain, UnitType};

#[derive(Clone, Copy)]
pub struct UnitEntity {
	pub entity: Entity,
	pub domain: UnitDomain,
	pub unit_type: UnitType,
	pub faction: Faction,
	pub position: Vec3,
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
		return self.tree.insert_pt(convert_to_point(pos.xz()), unit);
	}

	pub fn move_unit(&mut self, handle: u64, pos: Vec3) -> Option<u64> {
		if let Some(existing) = self.tree.get(handle) {
			if existing.anchor() == convert_to_point(pos.xz()) {
				return None;
			}
		} else {
			return None;
		}

		if let Some(entry) = self.tree.delete_by_handle(handle) {
			let p = convert_to_point(pos.xz());
			let mut entry = *entry.value_ref();
			entry.position = pos;
			return self.tree.insert_pt(p, entry);
		}
		return None;
	}

	pub fn get_units_in_circle(self, center: Vec3, radius: f32) -> Vec<Entity> {
		let anchor = center.xz() - Vec2::new(radius, radius);
		let d = (radius * 2.0) as usize;
		let area = AreaBuilder::default()
			.anchor(convert_to_point(anchor))
			.dimensions((d, d))
			.build()
			.unwrap();
		let query = self.tree.query(area);
		return query
			.filter(|e| e.value_ref().position.distance(center) <= radius)
			.map(|e| e.value_ref().entity)
			.collect();
	}

	pub fn get_units_in_rect(self, anchor: Vec2, size: Vec2) -> Vec<Entity> {
		let area = AreaBuilder::default()
			.anchor(convert_to_point(anchor))
			.dimensions((size.x as usize, size.y as usize))
			.build()
			.unwrap();
		let query = self.tree.query(area);
		return query.map(|e| e.value_ref().entity).collect();
	}
}

fn convert_to_point(pos: Vec2) -> Point<usize> {
	let p = pos.as_uvec2();
	return Point {
		x: p.x as usize,
		y: p.y as usize,
	};
}
