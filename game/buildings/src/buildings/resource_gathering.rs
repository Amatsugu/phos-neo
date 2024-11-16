use serde::{Deserialize, Serialize};
use shared::identifiers::ResourceIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceGatheringBuildingInfo {
	pub resources_to_gather: Vec<ResourceIdentifier>,
	pub gather_range: usize,
}
