use crate::{ValidationResult, traits::PlacementValidator, validators::Always};

#[derive(Default)]
pub struct NextToWater<T: PlacementValidator = Always>
{
	pub inner: T,
}

impl<T: PlacementValidator> PlacementValidator for NextToWater<T>
{
	type Inner = T;

	fn validate_self(
		&self,
		pos: hex::prelude::HexCoord,
		map: &world_generation::prelude::Map,
	) -> crate::ValidationResult
	{
		let neighboring_water = pos
			.get_neighbors()
			.iter()
			.any(|t| map.is_in_bounds(t) && map.is_underwater(t));
		ValidationResult {
			is_valid: neighboring_water,
			..Default::default()
		}
	}

	fn get_inner(&self) -> Option<&Self::Inner>
	{
		Some(&self.inner)
	}
}
