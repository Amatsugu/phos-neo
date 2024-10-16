use serde::{Deserialize, Serialize};

pub mod building;
pub mod despawn;
pub mod events;
pub mod identifiers;
pub mod resources;
pub mod sets;
pub mod states;
pub mod tags;

#[derive(Debug, Serialize, Deserialize)]
pub enum Tier {
	Zero,
	One,
	Two,
	Three,
	Superior,
}
