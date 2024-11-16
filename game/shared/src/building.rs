use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Default, Reflect, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildingIdentifier(pub usize);

impl From<i32> for BuildingIdentifier {
	fn from(value: i32) -> Self {
		return BuildingIdentifier(value as usize);
	}
}

impl From<u32> for BuildingIdentifier {
	fn from(value: u32) -> Self {
		return BuildingIdentifier(value as usize);
	}
}

impl From<usize> for BuildingIdentifier {
	fn from(value: usize) -> Self {
		return BuildingIdentifier(value);
	}
}

impl Into<usize> for BuildingIdentifier {
	fn into(self) -> usize {
		return self.0;
	}
}
