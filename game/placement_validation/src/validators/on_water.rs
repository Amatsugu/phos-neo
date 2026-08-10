use crate::{ValidationDisplay, traits::PlacementValidator, validators::Always};

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
		let on_water = map.is_underwater(&pos);
		crate::ValidationResult {
			is_valid: on_water,
			display: vec![ValidationDisplay::Tile {
				coord: pos,
				validation: match on_water {
					true => crate::ValidationType::Footprint,
					false => crate::ValidationType::Invalid,
				},
			}],
		}
	}

	fn get_inner(&self) -> Option<&Self::Inner>
	{
		Some(&self.inner)
	}
}
