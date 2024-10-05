use bevy::{ecs::world::CommandQueue, prelude::*, tasks::Task};
use serde::{Deserialize, Serialize};
use world_generation::hex_utils::HexCoord;

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
pub struct Target(pub HexCoord);

#[derive(Component, Debug)]
pub struct Path(pub Vec<Vec3>, pub usize);

#[derive(Component, Debug)]
pub struct PathTask(pub Task<Option<CommandQueue>>);
