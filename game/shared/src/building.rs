use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildingIdentifier(u32);

impl From<u32> for BuildingIdentifier {
	fn from(value: u32) -> Self {
		return BuildingIdentifier(value);
	}
}
