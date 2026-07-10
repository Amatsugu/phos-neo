use crate::{ValidationResult, traits::PlacementValidator, validators::Always};

#[derive(Default)]
pub struct Or<T1: PlacementValidator = Always, T2: PlacementValidator = Always>
{
	pub left: T1,
	pub right: T2,
}

impl<T1: PlacementValidator, T2: PlacementValidator> PlacementValidator for Or<T1, T2>
{
	type Inner = T1;

	fn validate_placement(
		&self,
		pos: hex::prelude::HexCoord,
		map: &world_generation::prelude::Map,
	) -> crate::ValidationResult
	{
		let left = self.left.validate_placement(pos, map);
		if left.is_valid {
			return left;
		}
		let right = self.right.validate_placement(pos, map);
		ValidationResult {
			is_valid: left.is_valid || right.is_valid,
			..(if left.is_valid {
				left
			} else if right.is_valid {
				right
			} else {
				left
			})
		}
	}

	fn validate_self(
		&self,
		_pos: hex::prelude::HexCoord,
		_map: &world_generation::prelude::Map,
	) -> crate::ValidationResult
	{
		ValidationResult::default()
	}

	fn get_inner(&self) -> Option<&Self::Inner>
	{
		Some(&self.left)
	}
}
