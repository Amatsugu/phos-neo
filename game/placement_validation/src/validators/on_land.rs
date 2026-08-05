use hex::prelude::HexCoord;
use world_generation::mapping::map::Map;

use crate::{ValidationDisplay, ValidationResult, traits::PlacementValidator, validators::Always};

#[derive(Default)]
pub struct OnLand<T: PlacementValidator = Always>
{
	pub inner: T,
}

impl<T: PlacementValidator> PlacementValidator for OnLand<T>
{
	type Inner = T;
	fn validate_self(&self, pos: HexCoord, map: &Map) -> ValidationResult
	{
		let on_land = map.is_on_land(&pos);
		ValidationResult {
			is_valid: on_land,
			display: vec![ValidationDisplay::Tile {
				coord: pos,
				validation: match on_land {
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
