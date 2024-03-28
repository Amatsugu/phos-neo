use crate::prelude::Chunk;
use bevy::prelude::*;

pub const OUTER_RADIUS: f32 = 1.;
pub const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866025404;

pub fn offset3d_to_world(offset: Vec3) -> Vec3 {
    let x = (offset.x + offset.z * 0.5 - (offset.z / 2.).floor()) * (INNER_RADIUS * 2.);
    return Vec3::new(x, offset.y, offset.z * OUTER_RADIUS * 1.5);
}

pub fn offset_to_world(offset: IVec2) -> Vec3 {
    let x = (offset.x as f32 + offset.y as f32 * 0.5 - (offset.y as f32 / 2.).floor())
        * (INNER_RADIUS * 2.);
    return Vec3::new(x, 0., offset.y as f32 * OUTER_RADIUS * 1.5);
}

pub fn offset_to_hex(offset: IVec2) -> IVec3 {
    return IVec3 {
        x: offset.x,
        y: offset.y,
        z: -offset.x - offset.y,
    };
}

pub fn snap_to_hex_grid(world_pos: Vec3) -> Vec3 {
    return offset_to_world(world_to_offset_pos(world_pos));
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

pub fn get_tile_count(radius: i32)->i32{
	return 1 + 3 * (radius + 1) * radius;
}

#[derive(Default, Clone, Copy)]
pub struct HexCoord {
    pub hex: IVec3,
    pub offset: IVec2,
    pub world: Vec3,
}

impl PartialEq<Self> for HexCoord {
    fn eq(&self, other: &Self) -> bool {
        return self.offset == other.offset;
    }
}

impl Eq for HexCoord {}

impl HexCoord {
    pub const DIRECTIONS: [IVec3; 6] = [
        IVec3::new(1, -1, 0),
        IVec3::new(1, 0, -1),
        IVec3::new(0, 1, -1),
        IVec3::new(-1, 1, 0),
        IVec3::new(-1, 0, 1),
        IVec3::new(0, -1, 1),
    ];

    pub const ZERO: HexCoord = HexCoord {
        offset: IVec2::ZERO,
        hex: IVec3::ZERO,
        world: Vec3::ZERO,
    };

    pub fn new(x: i32, z: i32) -> Self {
        return Self::from_offset(IVec2::new(x, z));
    }
    pub fn from_grid_pos(x: usize, z: usize) -> Self {
        return HexCoord::new(x as i32, z as i32);
    }
    pub fn from_offset(offset_pos: IVec2) -> Self {
        return HexCoord {
            offset: offset_pos,
            hex: offset_to_hex(offset_pos),
            world: offset3d_to_world(offset_pos.extend(0).xzy().as_vec3()),
        };
    }

    pub fn to_chunk_pos(&self) -> IVec2 {
        return IVec2 {
            x: (self.offset.x as f32 / Chunk::SIZE as f32).floor() as i32,
            y: (self.offset.y as f32 / Chunk::SIZE as f32).floor() as i32,
        };
    }

    pub fn to_chunk_index(&self, width: usize) -> usize {
        let pos = self.to_chunk_pos();
        return pos.x as usize + pos.y as usize * width;
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

	pub fn scale(&self, dir: i32, radius: i32)-> HexCoord{
		let s = Self::DIRECTIONS[dir % 6] * radius;
		return Self::from_offset(self.hex.xy() + s.xy());
	}

	pub fn get_neighbor(&self, dir: i32)-> HexCoord{
		let d = Self::DIRECTIONS[dir % 6];
		return Self::from_offset(self.hex.xy() + d.xy());
	}

	pub fn get_neighbors(&self) -> [HexCoord; 6]{
		return [
			self.get_neighbor(0),
			self.get_neighbor(1),
			self.get_neighbor(2),
			self.get_neighbor(3),
			self.get_neighbor(4),
			self.get_neighbor(5),
		]
	}
}
