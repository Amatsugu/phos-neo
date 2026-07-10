use crate::{ValidationResult, traits::PlacementValidator};

#[derive(Default)]
pub struct Always {}

impl PlacementValidator for Always
{
	type Inner = Always;

	fn validate_self(
		&self,
		_pos: hex::prelude::HexCoord,
		_map: &world_generation::prelude::Map,
	) -> crate::ValidationResult
	{
		ValidationResult {
			is_valid: true,
			..Default::default()
		}
	}

	fn get_inner(&self) -> Option<&Self::Inner>
	{
		None
	}
}
