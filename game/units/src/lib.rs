use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

pub mod assets;
pub mod components;
pub mod nav_data;
pub mod resources;
#[cfg(debug_assertions)]
pub mod units_debug_plugin;
pub mod units_plugin;
pub mod units_spacial_set;

#[derive(Clone, Copy)]
pub enum UnitType {
	Basic,
}
