use super::prelude::*;

#[test]
fn create_coord()
{
	let center = HexCoord::from_offset_pos(3, 3);
	for dir in 0..6
	{
		assert_eq!(center.get_neighbor(dir).get_neighbor(dir), center.scale(dir, 2));
	}
}
