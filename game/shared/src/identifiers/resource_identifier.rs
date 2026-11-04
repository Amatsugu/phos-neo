use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub struct ResourceIdentifier
{
	pub id: u64,
	pub qty: u32,
}
