use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct Unit;

#[derive(Component, Debug)]
pub struct AirUnit;
#[derive(Component, Debug)]
pub struct LandUnit;
#[derive(Component, Debug)]
pub struct NavalUnit;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UnitDomain {
	Land,
	Air,
	Naval,
}

#[derive(Component, Debug)]
pub struct Target(pub Vec3);
