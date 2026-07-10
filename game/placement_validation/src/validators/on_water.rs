use crate::{traits::PlacementValidator, validators::Always};

#[derive(Default)]
pub struct OnWater<T: PlacementValidator = Always>
{
	pub inner: T,
}

impl<T: PlacementValidator> PlacementValidator for OnWater<T>
{
	type Inner = T;

	fn validate_self(
		&self,
		pos: hex::prelude::HexCoord,
		map: &world_generation::prelude::Map,
	) -> crate::ValidationResult
	{
		crate::ValidationResult {
			is_valid: map.is_underwater(&pos),
			display: Vec::default(),
		}
	}

	fn get_inner(&self) -> Option<&Self::Inner>
	{
		Some(&self.inner)
	}
}
