use hex::prelude::{Chunk, HexCoord};
use world_generation::mapping::map::Map;

use crate::{
	traits::PlacementValidator,
	validators::{NextToWater, Not, OnLand, OnWater, Or},
};

#[test]
fn on_land()
{
	let map = create_test_map();
	let pos = HexCoord::from_offset_pos(1, 1);
	let v: OnLand = OnLand::default();

	let result = v.validate_placement(pos, &map);
	assert!(result.is_valid, "Validation failed: Expected on land");

	let pos = HexCoord::from_offset_pos(Chunk::SIZE - 1, Chunk::SIZE - 1);
	let result = v.validate_placement(pos, &map);
	assert!(!result.is_valid, "Validation failed: Expected on water");
}

#[test]
fn on_water()
{
	let map = create_test_map();
	let pos = HexCoord::from_offset_pos(Chunk::SIZE - 1, Chunk::SIZE - 1);
	let v: OnWater = Default::default();

	let result = v.validate_placement(pos, &map);
	assert!(result.is_valid, "Validation failed: Expected on water");

	let pos = HexCoord::from_offset_pos(1, 1);
	let result = v.validate_placement(pos, &map);
	assert!(!result.is_valid, "Validation failed: Expected on land");
}

#[test]
fn not()
{
	let map = create_test_map();
	let pos = HexCoord::from_offset_pos(Chunk::SIZE - 1, Chunk::SIZE - 1);

	let v: Not<OnLand> = Default::default();

	let result = v.validate_placement(pos, &map);
	assert!(result.is_valid, "Validation failed: Expected on water");

	let pos = HexCoord::from_offset_pos(1, 1);
	let result = v.validate_placement(pos, &map);
	assert!(!result.is_valid, "Validation failed: Expected on land");
}

#[test]
fn or()
{
	let map = create_test_map();
	let pos = HexCoord::from_offset_pos(Chunk::SIZE - 1, Chunk::SIZE - 1);

	let v: Or<OnLand, OnWater> = Default::default();

	let result = v.validate_placement(pos, &map);
	assert!(result.is_valid, "Validation failed: Expected on water");

	let pos = HexCoord::from_offset_pos(1, 1);
	let result = v.validate_placement(pos, &map);
	assert!(result.is_valid, "Validation failed: Expected on land");
}

#[test]
fn on_shore()
{
	let map = create_test_map();
	let pos = map
		.first(|c| {
			if map.is_on_land(c) {
				c.get_neighbors()
					.iter()
					.any(|n| map.is_in_bounds(n) && map.is_underwater(n))
			} else {
				false
			}
		})
		.expect("Map does not have any tiles next to water");
	let v: OnLand<NextToWater> = Default::default();

	let result = v.validate_placement(pos, &map);
	assert!(result.is_valid, "Validation failed: On shore");

	let pos = HexCoord::from_offset_pos(1, 1);
	let result = v.validate_placement(pos, &map);
	assert!(!result.is_valid, "Validation failed: Inland");
}

fn create_test_map() -> Map
{
	Map {
		biome_count: 0,
		chunks: vec![test_chunk()],
		height: 1,
		width: 1,
		max_level: 5.0,
		min_level: 0.0,
		sealevel: 2.0,
	}
}

fn test_chunk() -> Chunk
{
	let mut heights = [0.0; Chunk::AREA];
	for (idx, h) in heights.iter_mut().enumerate() {
		*h = ((idx as f32) / (Chunk::AREA as f32)) * 5.0;
	}
	Chunk {
		min_level: 0.0,
		max_level: 5.0,
		heights,
		..Default::default()
	}
}
