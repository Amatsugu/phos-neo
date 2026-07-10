use crate::{ValidationResult, traits::PlacementValidator, validators::Always};

#[derive(Default)]
pub struct Not<T: PlacementValidator = Always>
{
	pub inner: T,
}

impl<T: PlacementValidator> PlacementValidator for Not<T>
{
	type Inner = T;

	fn validate_placement(
		&self,
		pos: hex::prelude::HexCoord,
		map: &world_generation::prelude::Map,
	) -> crate::ValidationResult
	{
		if let Some(inner) = self.get_inner() {
			let mut res = inner.validate_placement(pos, map);
			res.is_valid = !res.is_valid;
			res
		} else {
			Default::default()
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
		Some(&self.inner)
	}
}
