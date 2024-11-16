use bevy::reflect::Reflect;
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

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub enum StatusEffect {
	UnitRange(f32),
	UnitAttack(f32),
	UnitHealth(f32),
	StructureRange(f32),
	StructureAttack(f32),
	StructureHealth(f32),
	BuildSpeedMulti(f32),
	BuildCostMulti(f32),
	ConsumptionMulti(f32),
	ProductionMulti(f32),
}
