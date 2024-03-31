use crate::prelude::Chunk;
use bevy::prelude::*;

pub const OUTER_RADIUS: f32 = 1.;
pub const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866025404;

pub fn offset3d_to_world(offset: Vec3) -> Vec3 {
	let x = (offset.x + (offset.z * 0.5) - (offset.z / 2.).floor()) * (INNER_RADIUS * 2.);
	return Vec3::new(x, offset.y, offset.z * OUTER_RADIUS * 1.5);
}

pub fn offset_to_world(offset: IVec2, height: f32) -> Vec3 {
	let off = offset.as_vec2();
	let x = (off.x + (off.y * 0.5) - (off.y / 2.).floor()) * (INNER_RADIUS * 2.);
	return Vec3::new(x, height, off.y * OUTER_RADIUS * 1.5);
}

pub fn offset_to_hex(offset: IVec2) -> IVec3 {
	return IVec3 {
		x: offset.x,
		y: offset.y,
		z: -offset.x - offset.y,
	};
}

pub fn snap_to_hex_grid(world_pos: Vec3) -> Vec3 {
	return offset_to_world(world_to_offset_pos(world_pos), world_pos.y);
}

pub fn world_to_offset_pos(world_pos: Vec3) -> IVec2 {
	let offset = world_pos.z / (OUTER_RADIUS * 3.);
	let x = (world_pos.x / (INNER_RADIUS * 2.)) - offset;
	let z = -world_pos.x - offset;

	let ix = x.round() as i32;
	let iz = z.round() as i32;
	let ox = ix + iz / 2;
	let oz = iz;
	return IVec2::new(ox, oz);
}

pub fn tile_to_world_distance(dist: i32) -> f32 {
	return dist as f32 * (2. * INNER_RADIUS);
}

pub fn get_tile_count(radius: i32) -> i32 {
	return 1 + 3 * (radius + 1) * radius;
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct HexCoord {
	pub hex: IVec3,
}

impl HexCoord {
	pub const DIRECTIONS: [IVec3; 6] = [
		IVec3::new(1, 1, 0),
		IVec3::new(1, 0, -1),
		IVec3::new(1, -1, 0),
		IVec3::new(0, -1, 1),
		IVec3::new(-1, 0, 1),
		IVec3::new(0, 1, -1),
	];

	pub const ZERO: HexCoord = HexCoord { hex: IVec3::ZERO };

	pub fn new(x: i32, z: i32) -> Self {
		return Self::from_offset(IVec2::new(x, z));
	}
	pub fn from_grid_pos(x: usize, z: usize) -> Self {
		return HexCoord::new(x as i32, z as i32);
	}
	pub fn from_offset(offset_pos: IVec2) -> Self {
		return HexCoord {
			hex: offset_to_hex(offset_pos),
		};
	}

	pub fn is_in_bounds(&self, map_height: usize, map_width: usize) -> bool {
		if self.hex.x < 0 || self.hex.y < 0 {
			return false;
		}

		if self.hex.x >= map_width as i32 || self.hex.y >= map_height as i32 {
			return false;
		}

		return true;
	}

	pub fn to_chunk_pos(&self) -> IVec2 {
		return IVec2 {
			x: (self.hex.x as f32 / Chunk::SIZE as f32).floor() as i32,
			y: (self.hex.y as f32 / Chunk::SIZE as f32).floor() as i32,
		};
	}

	pub fn to_chunk(&self) -> HexCoord {
		let c_pos = self.to_chunk_pos();
		return HexCoord::from_offset(
			(
				self.hex.x - (c_pos.x * Chunk::SIZE as i32),
				self.hex.y - (c_pos.y * Chunk::SIZE as i32),
			)
				.into(),
		);
	}

	pub fn to_world(&self, height: f32) -> Vec3 {
		return offset_to_world(self.hex.xy(), height);
	}

	pub fn to_offset(&self) -> IVec2 {
		return self.hex.xy();
	}

	pub fn to_index(&self, width: usize) -> i32 {
		return self.hex.x + self.hex.y * width as i32 + self.hex.y / 2;
	}
	pub fn to_chunk_index(&self, width: usize) -> i32 {
		let pos = self.to_chunk_pos();
		return pos.x + pos.y * width as i32;
	}

	pub fn to_chunk_local_index(&self) -> i32 {
		return self.to_chunk().to_index(Chunk::SIZE);
	}

	pub fn distance(&self, other: &HexCoord) -> i32 {
		return (self.hex.x - other.hex.x).abs()
			+ (self.hex.y - other.hex.y).abs()
			+ (self.hex.z - other.hex.z).abs();
	}

	pub fn rotate_around(&self, center: &HexCoord, angle: i32) -> HexCoord {
		if self == center || angle == 0 {
			return self.clone();
		}

		let mut a = angle % 6;
		let mut pc = self.hex - center.hex;

		if a > 0 {
			for _ in 0..a {
				pc = Self::slide_right(pc);
			}
		} else {
			a = a.abs();
			for _ in 0..a {
				pc = Self::slide_left(pc);
			}
		}
		return HexCoord::from_offset(pc.xy() + center.hex.xy());
	}

	fn slide_left(hex: IVec3) -> IVec3 {
		return (hex * -1).yzx();
	}

	fn slide_right(hex: IVec3) -> IVec3 {
		return (hex * -1).zxy();
	}

	pub fn scale(&self, dir: i32, radius: usize) -> HexCoord {
		let s = Self::DIRECTIONS[(dir % 6) as usize] * radius as i32;
		return Self::from_offset(self.hex.xy() + s.xy());
	}

	pub fn get_neighbor(&self, dir: usize) -> HexCoord {
		let d = Self::DIRECTIONS[dir % 6];
		return Self::from_offset(self.hex.xy() + d.xy());
	}

	pub fn get_neighbors(&self) -> [HexCoord; 6] {
		return [
			self.get_neighbor(0),
			self.get_neighbor(1),
			self.get_neighbor(2),
			self.get_neighbor(3),
			self.get_neighbor(4),
			self.get_neighbor(5),
		];
	}
}
