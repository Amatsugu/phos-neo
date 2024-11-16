use serde::{Deserialize, Serialize};
use shared::{building::BuildingIdentifier, StatusEffect};

#[derive(Serialize, Deserialize, Debug)]
pub struct TechBuildingInfo {
	pub effect_range: usize,
	pub buildings_to_unlock: Vec<BuildingIdentifier>,
	pub buffs: Vec<StatusEffect>,
}
