use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Default, Reflect, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct BuildingIdentifier(pub u64);

impl From<&String> for BuildingIdentifier
{
	fn from(value: &String) -> Self
	{
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);

		return Self(hasher.finish());
	}
}

impl From<&str> for BuildingIdentifier
{
	fn from(value: &str) -> Self
	{
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);

		return Self(hasher.finish());
	}
}
