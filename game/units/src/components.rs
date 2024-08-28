use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct Unit;

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UnitDomain {
	Land,
	Air,
	Navy,
}
