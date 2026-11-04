use serde::{Deserialize, Serialize};
use shared::{StatusEffect, identifiers::BuildingIdentifier};

#[derive(Serialize, Deserialize, Debug)]
pub struct TechBuildingInfo
{
	pub effect_range: usize,
	pub buildings_to_unlock: Vec<BuildingIdentifier>,
	pub buffs: Vec<StatusEffect>,
}
