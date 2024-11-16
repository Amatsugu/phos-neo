use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FactoryBuildingInfo {
	pub units_to_build: Vec<()>
}
