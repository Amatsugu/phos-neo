use hex::prelude::HexCoord;
use world_generation::mapping::map::Map;

use crate::ValidationResult;

pub trait PlacementValidator
{
	type Inner: PlacementValidator;
	fn validate_placement(&self, pos: HexCoord, map: &Map) -> ValidationResult
	{
		let result = self.validate_self(pos, map);
		if let Some(inner) = self.get_inner() {
			inner.validate_placement(pos, map).merge_with(result)
		} else {
			result
		}
	}
	fn validate_self(&self, pos: HexCoord, map: &Map) -> ValidationResult;

	fn get_inner(&self) -> Option<&Self::Inner>;
}
